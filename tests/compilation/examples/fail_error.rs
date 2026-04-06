use proc_macro2_diagnostic_fixture::error_no_help;

#[error_no_help]
fn foo() {
    todo!()
}

fn main() {}
