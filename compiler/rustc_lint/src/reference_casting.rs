use rustc_ast::Mutability;
use rustc_hir::{Expr, ExprKind, UnOp};
use rustc_middle::ty::{self, TypeAndMut};
use rustc_span::sym;

use crate::{lints::InvalidReferenceCastingDiag, LateContext, LateLintPass, LintContext};

declare_lint! {
    /// The `invalid_reference_casting` lint checks for casts of `&T` to `&mut T`
    /// without using interior mutability.
    ///
    /// ### Example
    ///
    /// ```rust,compile_fail
    /// fn x(r: &i32) {
    ///     unsafe {
    ///         *(r as *const i32 as *mut i32) += 1;
    ///     }
    /// }
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// Casting `&T` to `&mut T` without using interior mutability is undefined behavior,
    /// as it's a violation of Rust reference aliasing requirements.
    ///
    /// `UnsafeCell` is the only way to obtain aliasable data that is considered
    /// mutable.
    INVALID_REFERENCE_CASTING,
    Deny,
    "casts of `&T` to `&mut T` without interior mutability"
}

declare_lint_pass!(InvalidReferenceCasting => [INVALID_REFERENCE_CASTING]);

impl<'tcx> LateLintPass<'tcx> for InvalidReferenceCasting {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let Some((is_assignment, e)) = is_operation_we_care_about(cx, expr) else {
            return;
        };

        let init = cx.expr_or_init(e);

        let orig_cast = if is_cast_from_const_to_mut(cx, init) {
            if init.span != e.span { Some(init.span) } else { None }
        } else {
            return;
        };

        cx.emit_spanned_lint(
            INVALID_REFERENCE_CASTING,
            expr.span,
            if is_assignment {
                InvalidReferenceCastingDiag::AssignToRef { orig_cast }
            } else {
                InvalidReferenceCastingDiag::BorrowAsMut { orig_cast }
            },
        );
    }
}

fn is_operation_we_care_about<'tcx>(
    cx: &LateContext<'tcx>,
    e: &'tcx Expr<'tcx>,
) -> Option<(bool, &'tcx Expr<'tcx>)> {
    fn deref_assign_or_addr_of<'tcx>(expr: &'tcx Expr<'tcx>) -> Option<(bool, &'tcx Expr<'tcx>)> {
        // &mut <expr>
        let inner = if let ExprKind::AddrOf(_, Mutability::Mut, expr) = expr.kind {
            expr
        // <expr> = ...
        } else if let ExprKind::Assign(expr, _, _) = expr.kind {
            expr
        // <expr> += ...
        } else if let ExprKind::AssignOp(_, expr, _) = expr.kind {
            expr
        } else {
            return None;
        };

        if let ExprKind::Unary(UnOp::Deref, e) = &inner.kind {
            Some((!matches!(expr.kind, ExprKind::AddrOf(..)), e))
        } else {
            None
        }
    }

    fn ptr_write<'tcx>(
        cx: &LateContext<'tcx>,
        e: &'tcx Expr<'tcx>,
    ) -> Option<(bool, &'tcx Expr<'tcx>)> {
        if let ExprKind::Call(path, [arg_ptr, _arg_val]) = e.kind
            && let ExprKind::Path(ref qpath) = path.kind
            && let Some(def_id) = cx.qpath_res(qpath, path.hir_id).opt_def_id()
            && matches!(cx.tcx.get_diagnostic_name(def_id), Some(sym::ptr_write | sym::ptr_write_volatile | sym::ptr_write_unaligned))
        {
            Some((true, arg_ptr))
        } else {
            None
        }
    }

    deref_assign_or_addr_of(e).or_else(|| ptr_write(cx, e))
}

fn is_cast_from_const_to_mut<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'tcx>) -> bool {
    let e = e.peel_blocks();

    fn from_casts<'tcx>(
        cx: &LateContext<'tcx>,
        e: &'tcx Expr<'tcx>,
        need_check_freeze: &mut bool,
    ) -> Option<&'tcx Expr<'tcx>> {
        // <expr> as *mut ...
        let mut e = if let ExprKind::Cast(e, t) = e.kind
            && let ty::RawPtr(TypeAndMut { mutbl: Mutability::Mut, .. }) = cx.typeck_results().node_type(t.hir_id).kind() {
            e
        // <expr>.cast_mut()
        } else if let ExprKind::MethodCall(_, expr, [], _) = e.kind
            && let Some(def_id) = cx.typeck_results().type_dependent_def_id(e.hir_id)
            && cx.tcx.is_diagnostic_item(sym::ptr_cast_mut, def_id) {
            expr
        // UnsafeCell::raw_get(<expr>)
        } else if let ExprKind::Call(path, [arg]) = e.kind
            && let ExprKind::Path(ref qpath) = path.kind
            && let Some(def_id) = cx.qpath_res(qpath, path.hir_id).opt_def_id()
            && cx.tcx.is_diagnostic_item(sym::unsafe_cell_raw_get, def_id)
        {
            *need_check_freeze = true;
            arg
        } else {
            return None;
        };

        let mut had_at_least_one_cast = false;
        loop {
            e = e.peel_blocks();
            // <expr> as *mut/const ... or <expr> as <uint>
            e = if let ExprKind::Cast(expr, t) = e.kind
                && matches!(cx.typeck_results().node_type(t.hir_id).kind(), ty::RawPtr(_) | ty::Uint(_))  {
                had_at_least_one_cast = true;
                expr
            // <expr>.cast(), <expr>.cast_mut() or <expr>.cast_const()
            } else if let ExprKind::MethodCall(_, expr, [], _) = e.kind
                && let Some(def_id) = cx.typeck_results().type_dependent_def_id(e.hir_id)
                && matches!(
                    cx.tcx.get_diagnostic_name(def_id),
                    Some(sym::ptr_cast | sym::const_ptr_cast | sym::ptr_cast_mut | sym::ptr_cast_const)
                )
            {
                had_at_least_one_cast = true;
                expr
            // ptr::from_ref(<expr>) or UnsafeCell::raw_get(<expr>)
            } else if let ExprKind::Call(path, [arg]) = e.kind
                && let ExprKind::Path(ref qpath) = path.kind
                && let Some(def_id) = cx.qpath_res(qpath, path.hir_id).opt_def_id()
                && matches!(
                    cx.tcx.get_diagnostic_name(def_id),
                    Some(sym::ptr_from_ref | sym::unsafe_cell_raw_get)
                )
            {
                if cx.tcx.is_diagnostic_item(sym::unsafe_cell_raw_get, def_id) {
                    *need_check_freeze = true;
                }
                return Some(arg);
            } else if had_at_least_one_cast {
                return Some(e);
            } else {
                return None;
            };
        }
    }

    fn from_transmute<'tcx>(
        cx: &LateContext<'tcx>,
        e: &'tcx Expr<'tcx>,
    ) -> Option<&'tcx Expr<'tcx>> {
        // mem::transmute::<_, *mut _>(<expr>)
        if let ExprKind::Call(path, [arg]) = e.kind
            && let ExprKind::Path(ref qpath) = path.kind
            && let Some(def_id) = cx.qpath_res(qpath, path.hir_id).opt_def_id()
            && cx.tcx.is_diagnostic_item(sym::transmute, def_id)
            && let ty::RawPtr(TypeAndMut { mutbl: Mutability::Mut, .. }) = cx.typeck_results().node_type(e.hir_id).kind() {
            Some(arg)
        } else {
            None
        }
    }

    let mut need_check_freeze = false;
    let Some(e) = from_casts(cx, e, &mut need_check_freeze).or_else(|| from_transmute(cx, e))
    else {
        return false;
    };

    let e = e.peel_blocks();
    let node_type = cx.typeck_results().node_type(e.hir_id);
    if let ty::Ref(_, inner_ty, Mutability::Not) = node_type.kind() {
        // If an UnsafeCell method is involved we need to additionaly check the
        // inner type for the presence of the Freeze trait (ie does NOT contain
        // an UnsafeCell), since in that case we would incorrectly lint on valid casts.
        //
        // We also consider non concrete skeleton types (ie generics)
        // to be an issue since there is no way to make it safe for abitrary types.
        !need_check_freeze
            || inner_ty.is_freeze(cx.tcx, cx.param_env)
            || !inner_ty.has_concrete_skeleton()
    } else {
        false
    }
}
