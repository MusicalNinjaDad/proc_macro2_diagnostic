use trybuild::{self, TestCases};

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail("tests/compilation/tests/*.rs");
}
