use proc_macro2_diagnostic_fixture::{combined_syn_errors, convert_syn_error};

convert_syn_error!(Valid);
convert_syn_error!(1);
combined_syn_errors!(AlsoValid);
combined_syn_errors!(1);

fn main() {
    let _foo: Valid = Valid {};
    let _bar: AlsoValid = AlsoValid {};
}
