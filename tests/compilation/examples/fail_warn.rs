use proc_macro2_diagnostic_fixture::{error, warn};

#[error]
// Trybuild needs a compiler error, that doesn't kill clippy.
// it won't fail on just a warning
fn foo() {
    todo!()
}

warn!();

fn main() {
    let _foo: warn = warn {};
}
