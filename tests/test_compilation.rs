use trybuild::{self, TestCases};

#[test]
fn failures() {
    let t = TestCases::new();
    #[cfg(has_proc_macro_diagnostic)]
    t.compile_fail("tests/compilation/examples/fail_*.rs");
    #[cfg(not(has_proc_macro_diagnostic))]
    t.compile_fail("tests/compilation/examples/no_diagnostic/fail_*.rs");
}

#[test]
fn ok() {
    let t = TestCases::new();
    #[cfg(has_proc_macro_diagnostic)]
    t.pass("tests/compilation/examples/pass_*.rs");
    #[cfg(not(has_proc_macro_diagnostic))]
    t.pass("tests/compilation/examples/no_diagnostic/pass_*.rs");
}
