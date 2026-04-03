use proc_macro2_diagnostic_fixture::error_and_help;

#[error_and_help]
fn foo() {
    todo!()
}

fn main() {}
