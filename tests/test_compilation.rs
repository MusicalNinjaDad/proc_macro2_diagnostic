use trybuild::{self, TestCases};

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail("tests/compilation/examples/fail_*.rs");
}

#[test]
fn ok() {
    let t = TestCases::new();
    t.pass("tests/compilation/examples/pass_*.rs");
}
