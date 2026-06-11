use ninja_build_rs::{Result, nightly::Nightly, split_var};

fn main() -> Result<()> {
    let ac = autocfg::new();
    ac.emit_unstable_feature("assert_matches");
    let test_flags = split_var("PROC_MACRO2_DIAGNOSTIC_TEST").unwrap_or_default();
    if test_flags.contains("no_diagnostic") {
        autocfg::emit_possibility("unstable_proc_macro_diagnostic");
        autocfg::emit_possibility("has_proc_macro_diagnostic");
    } else {
        ac.emit_unstable_feature("proc_macro_diagnostic");
    }
    if test_flags.contains("no_try") {
        autocfg::emit_possibility("unstable_never_type");
        autocfg::emit_possibility("has_never_type");
        autocfg::emit_possibility("unstable_try_collect");
        autocfg::emit_possibility("has_try_collect");
        autocfg::emit_possibility("unstable_try_trait_v2");
        autocfg::emit_possibility("has_try_trait_v2");
        autocfg::emit_possibility("unstable_try_trait_v2_residual");
        autocfg::emit_possibility("has_try_trait_v2_residual");
    } else {
        ac.emit_unstable_feature("never_type");
        ac.emit_unstable_feature("try_collect");
        ac.emit_unstable_feature("try_trait_v2");
        ac.emit_unstable_feature("try_trait_v2_residual");
    }
    Ok(())
}
