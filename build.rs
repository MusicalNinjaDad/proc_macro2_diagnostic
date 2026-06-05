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
    Ok(())
}
