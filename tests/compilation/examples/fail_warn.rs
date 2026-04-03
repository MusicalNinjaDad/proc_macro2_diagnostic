use proc_macro2_diagnostic_fixture::warn;

warn!();

fn main() {
    let _foo: warn = warn {};
    trybuild does not fail on warnings
}
