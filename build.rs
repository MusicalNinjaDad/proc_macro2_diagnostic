use build_safely::prelude::*;

fn main() -> Result<()> {
    let mut ac = AutoCfg::new()?;
    ac.set_edition(Some("2024".to_string()));

    let allowed_features = cargo_allowed_features()?;
    ac.emit_unstable_feature(assert_matches, &allowed_features);

    ac.emit_unstable_feature(proc_macro_diagnostic, &allowed_features);

    ac.emit_unstable_feature(iterator_try_collect, &allowed_features);
    ac.emit_unstable_feature(never_type, &allowed_features);
    ac.emit_unstable_feature(try_trait_v2, &allowed_features);
    ac.emit_unstable_feature(try_trait_v2_residual, &allowed_features);

    Ok(())
}
