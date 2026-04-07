use proc_macro2_diagnostic_fixture::convert_syn_error;

convert_syn_error!(Valid);
convert_syn_error!(1);

fn main() {
    let _foo: Valid = Valid {};
}
