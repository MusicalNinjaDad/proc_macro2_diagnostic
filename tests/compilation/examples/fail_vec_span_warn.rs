use proc_macro2_diagnostic_fixture::{error, vec_span_warn};

#[error]
// Trybuild needs a compiler error, that doesn't kill clippy.
// it won't fail on just a warning
fn foo() {
    todo!()
}

vec_span_warn!(foo bar baz);

fn main() {
    let _foo: VecSpanWarn = VecSpanWarn;
}
