use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::{expr_or_init, in_constant};
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty;
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::symbol::sym;

declare_clippy_lint! {
    /// ### What it does
    /// When `a` is `&[T]`, detect `a.len() * size_of::<T>()` and suggest `size_of_val(a)`
    /// instead.
    ///
    /// ### Why is this better?
    /// * Shorter to write
    /// * Removes the need for the human and the compiler to worry about overflow in the
    ///   multiplication
    /// * Potentially faster at runtime as rust emits special no-wrapping flags when it
    ///   calculates the byte length
    /// * Less turbofishing
    ///
    /// ### Example
    /// ```rust
    /// # let data : &[i32] = &[1, 2, 3];
    /// let newlen = data.len() * std::mem::size_of::<i32>();
    /// ```
    /// Use instead:
    /// ```rust
    /// # let data : &[i32] = &[1, 2, 3];
    /// let newlen = std::mem::size_of_val(data);
    /// ```
    #[clippy::version = "1.70.0"]
    pub MANUAL_SLICE_SIZE_CALCULATION,
    complexity,
    "manual slice size calculation"
}
declare_lint_pass!(ManualSliceSizeCalculation => [MANUAL_SLICE_SIZE_CALCULATION]);

impl<'tcx> LateLintPass<'tcx> for ManualSliceSizeCalculation {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // Does not apply inside const because size_of_value is not cost in stable.
        if !in_constant(cx, expr.hir_id)
            && let ExprKind::Binary(ref op, left, right) = expr.kind
            && BinOpKind::Mul == op.node
            && let Some(_receiver) = simplify(cx, left, right)
        {
            span_lint_and_help(
                cx,
                MANUAL_SLICE_SIZE_CALCULATION,
                expr.span,
                "manual slice size calculation",
                None,
                "consider using std::mem::size_of_value instead");
        }
    }
}

fn simplify<'tcx>(
    cx: &LateContext<'tcx>,
    expr1: &'tcx Expr<'tcx>,
    expr2: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    let expr1 = expr_or_init(cx, expr1);
    let expr2 = expr_or_init(cx, expr2);

    simplify_half(cx, expr1, expr2).or_else(|| simplify_half(cx, expr2, expr1))
}

fn simplify_half<'tcx>(
    cx: &LateContext<'tcx>,
    expr1: &'tcx Expr<'tcx>,
    expr2: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    if
        // expr1 is `[T1].len()`?
        let ExprKind::MethodCall(method_path, receiver, _, _) = expr1.kind
        && method_path.ident.name == sym::len
        && let receiver_ty = cx.typeck_results().expr_ty(receiver)
        && let ty::Slice(ty1) = receiver_ty.peel_refs().kind()
        // expr2 is `size_of::<T2>()`?
        && let ExprKind::Call(func, _) = expr2.kind
        && let ExprKind::Path(ref func_qpath) = func.kind
        && let Some(def_id) = cx.qpath_res(func_qpath, func.hir_id).opt_def_id()
        && cx.tcx.is_diagnostic_item(sym::mem_size_of, def_id)
        && let Some(ty2) = cx.typeck_results().node_substs(func.hir_id).types().next()
        // T1 == T2?
        && *ty1 == ty2
    {
        Some(receiver)
    } else {
        None
    }
}
