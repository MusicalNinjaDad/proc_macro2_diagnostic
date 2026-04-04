use proc_macro2_diagnostic_fixture::multi_span_warn;

fn main() {
    multi_span_warn!(foo bar baz);
}