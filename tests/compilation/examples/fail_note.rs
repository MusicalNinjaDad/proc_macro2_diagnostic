use proc_macro2_diagnostic_fixture::{error_no_help, just_a_note};

#[error_no_help]
// Trybuild needs a compiler error, that doesn't kill clippy.
// it won't fail on just a warning
fn foo() {
    todo!()
}

just_a_note!();

fn main() {
    let _foo: Bob = Bob {};
}
