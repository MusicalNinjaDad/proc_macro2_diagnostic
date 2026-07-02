use ninja_build_rs::{
    Result,
    nightly::{Nightly, cargo_allowed_features},
    split_var,
};

fn main() -> Result<()> {
    let mut ac = autocfg::new();
    ac.set_edition(Some("2024".to_string()));

    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature("assert_matches", &allowed_features);
    let test_flags = split_var("PROC_MACRO2_DIAGNOSTIC_TEST").unwrap_or_default();
    if test_flags.contains("no_diagnostic") {
        autocfg::emit_possibility("unstable_proc_macro_diagnostic");
        autocfg::emit_possibility("has_proc_macro_diagnostic");
    } else {
        ac.emit_unstable_feature("proc_macro_diagnostic", &allowed_features);
    }
    if test_flags.contains("no_try") {
        autocfg::emit_possibility("unstable_iterator_try_collect");
        autocfg::emit_possibility("has_iterator_try_collect");
        autocfg::emit_possibility("unstable_never_type");
        autocfg::emit_possibility("has_never_type");
        autocfg::emit_possibility("unstable_try_trait_v2");
        autocfg::emit_possibility("has_try_trait_v2");
        autocfg::emit_possibility("unstable_try_trait_v2_residual");
        autocfg::emit_possibility("has_try_trait_v2_residual");
    } else {
        ac.emit_unstable_feature("iterator_try_collect", &allowed_features);
        ac.emit_unstable_feature("never_type", &allowed_features);
        ac.emit_unstable_feature("try_trait_v2", &allowed_features);
        ac.emit_unstable_feature("try_trait_v2_residual", &allowed_features);
    }
    Ok(())
}
