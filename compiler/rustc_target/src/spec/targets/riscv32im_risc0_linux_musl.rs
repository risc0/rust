use std::borrow::Cow;

use crate::spec::{CodeModel, SplitDebuginfo, Target, TargetMetadata, TargetOptions, base};

pub(crate) fn target() -> Target {
    Target {
        // llvm_target: "riscv32".into(),
        llvm_target: "riscv32-unknown-linux-musl".into(),
        metadata: TargetMetadata {
            // description: Some("RISC Zero's zero-knowledge Virtual Machine (RV32IM ISA)".into()),
            description: Some(
                "RISC-V Linux (kernel 5.4, musl 1.2.3 + RISCV32 support patches".into(),
            ),
            tier: Some(3),
            host_tools: Some(false),
            // std: None, // ?
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
            llvm_abiname: "ilp32d".into(),
            // max_atomic_width: Some(64),
            max_atomic_width: Some(32),
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            // FIXME(compiler-team#422): musl targets should be dynamically linked by default.
            crt_static_default: true,

            // risc0 overrides
            singlethread: true,
            vendor: "risc0".into(),
            // panic_strategy: PanicStrategy::Abort,
            // linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            // linker: Some("rust-lld".into()),
            // relocation_model: RelocModel::Static,
            // emit_debug_gdb_scripts: false,
            // eh_frame_header: false,

            // linux_base.rs
            // base.env = "musl".into();
            // base.pre_link_objects_self_contained = crt_objects::pre_musl_self_contained();
            // base.post_link_objects_self_contained = crt_objects::post_musl_self_contained();
            // base.link_self_contained = LinkSelfContainedDefault::InferredForMusl;

            // linux.rs
            // os: "linux".into(),
            // dynamic_linking: true,
            // families: cvs!["unix"],
            // has_rpath: true,
            // position_independent_executables: true,
            // relro_level: RelroLevel::Full,
            // has_thread_local: true,
            // crt_static_respected: true,
            // supported_split_debuginfo: Cow::Borrowed(&[
            //     SplitDebuginfo::Packed,
            //     SplitDebuginfo::Unpacked,
            //     SplitDebuginfo::Off,
            // ]),
            ..base::linux_musl::opts()
        },
    }
}
