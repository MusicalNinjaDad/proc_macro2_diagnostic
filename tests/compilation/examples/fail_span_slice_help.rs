use proc_macro2_diagnostic_fixture::{error_no_help, span_slice_help};

#[error_no_help]
// Trybuild needs a compiler error, that doesn't kill clippy.
// it won't fail on just a warning
fn foo() {
    todo!()
}

span_slice_help!(help1 help2);

fn main() {
    let _foo: SpanSliceHelp = SpanSliceHelp;
}
