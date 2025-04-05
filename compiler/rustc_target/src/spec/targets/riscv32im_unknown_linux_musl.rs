use std::borrow::Cow;

use crate::spec::Cc;
use crate::spec::LinkerFlavor;
use crate::spec::Lld;
use crate::spec::RelocModel;
// use crate::spec::PanicStrategy;
use crate::spec::{CodeModel, SplitDebuginfo, Target, TargetMetadata, TargetOptions, base};

pub(crate) fn target() -> Target {
    Target {
        // llvm_target: "riscv32".into(),
        // llvm_target: "riscv32-unknown-linux-gnu".into(),
        llvm_target: "riscv32-unknown-linux-musl".into(),
        metadata: TargetMetadata {
            description: Some("RISC-V Linux (kernel 4.19, musl 1.2.5)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i64:64-n32-S128".into(),
        arch: "riscv32".into(),
        options: TargetOptions {
            code_model: Some(CodeModel::Medium),
            cpu: "generic-rv32".into(),
            // features: "+m,+a,+f,+d,+c".into(),
            features: "+m".into(),
            // llvm_abiname: "ilp32d".into(),
            llvm_abiname: "ilp32".into(),
            max_atomic_width: Some(32),
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),

            // risc0 overrides
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            linker: Some("rust-lld".into()),
            // panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            // emit_debug_gdb_scripts: false,
            // eh_frame_header: false,
            singlethread: true,
            crt_static_default: true,

            // base
            ..base::linux_musl::opts()
        },
    }
}
