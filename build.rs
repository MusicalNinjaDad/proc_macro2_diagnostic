use ninja_build_rs::{Result, get_var, nightly::Nightly};

fn main() -> Result<()> {
    let ac = autocfg::new();
    ac.emit_unstable_feature("assert_matches");
    ac.emit_unstable_feature("never_type");
    if get_var("PROC_MACRO2_DIAGNOSTIC_TEST").is_ok() {
        autocfg::emit_possibility("unstable_proc_macro_diagnostic");
        autocfg::emit_possibility("has_proc_macro_diagnostic");
    } else {
        ac.emit_unstable_feature("proc_macro_diagnostic");
    }
    ac.emit_unstable_feature("try_trait_v2");
    ac.emit_unstable_feature("try_trait_v2_residual");
    Ok(())
}
