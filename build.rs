use ninja_build_rs::{Result, nightly::Nightly};

fn main() -> Result<()> {
    let ac = autocfg::new();
    ac.emit_unstable_feature("assert_matches");
    ac.emit_unstable_feature("never_type");
    ac.emit_unstable_feature("proc_macro_diagnostic");
    ac.emit_unstable_feature("try_trait_v2");
    ac.emit_unstable_feature("try_trait_v2_residual");
    Ok(())
}
