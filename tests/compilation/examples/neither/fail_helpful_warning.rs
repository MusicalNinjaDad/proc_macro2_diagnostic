use proc_macro2_diagnostic_fixture::{error_no_help, helpful_warning};

#[error_no_help]
// Trybuild needs a compiler error, that doesn't kill clippy.
// it won't fail on just a warning
fn foo() {
    todo!()
}

helpful_warning!();

fn main() {
    let _foo: helpful_warning = helpful_warning {};
}
