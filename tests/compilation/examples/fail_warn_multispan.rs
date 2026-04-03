use proc_macro2_diagnostic_fixture::{error, warn_multispan};

#[error]
// Trybuild needs a compiler error, that doesn't kill clippy.
// it won't fail on just a warning
fn foo() {
    todo!()
}

#[derive(warn_multispan)]
#[repr(u8)]
enum Foo {
    Bar,
    Snort,
}

fn main() {}
