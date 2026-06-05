use autocfg::AutoCfg;
use ninja_build_rs::{
    Result,
    nightly::{AssertMatchesLocation, Nightly},
};

fn main() -> Result<()> {
    let ac = autocfg::new();
    ac.emit_unstable_feature("assert_matches");
    AssertMatchesLocation::emit_possibilities();
    if let Some(location) = ac.assert_matches_location() {
        autocfg::emit(&location.to_string())
    }
    ac.emit_unstable_feature("never_type");
    ac.emit_unstable_feature("proc_macro_diagnostic");
    emit_unstable_pmdiag(&ac);
    ac.emit_unstable_feature("try_trait_v2");
    ac.emit_unstable_feature("try_trait_v2_residual");
    Ok(())
}

fn emit_unstable_pmdiag(ac: &AutoCfg) {
    let feature = "proc_macro_diagnostic";
    let cfg = format!("unstable_{feature}");
    // #![allow(unused)] is required to avoid this failing for `cargo clippy -- -D warnings`
    let code = format!(
        r#"
        #![deny(stable_features)]
        #![allow(unused)]
        #![feature({feature})]
        extern crate proc_macro;
        "#
    );
    dbg!(&code);
    autocfg::emit_possibility(&cfg);
    if ac.probe_raw(&code).is_ok() {
        autocfg::emit(&cfg);
    }
}
