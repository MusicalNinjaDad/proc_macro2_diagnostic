use proc_macro2_diagnostic_fixture::oops;

#[oops]
fn foo() {
    todo!()
}

fn main() {}
