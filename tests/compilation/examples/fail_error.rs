use proc_macro2_diagnostic_fixture::error;

#[error]
fn foo() {
    todo!()
}

fn main() {}
