use std::path::PathBuf;

use trybuild::{self, TestCases};

#[expect(clippy::no_effect)]
fn examples(path: &str) -> PathBuf {
    let mut examples = PathBuf::from("tests/compilation/examples");
    cfg_select! {
        not(has_proc_macro_diagnostic) => examples.push("no_diagnostic"),
        not(has_try_trait_v2) => examples.push("no_try"),
        _ => (),
    };
    examples.push(path);
    examples
}

#[test]
fn failures() {
    let t = TestCases::new();
    t.compile_fail(examples("fail_*.rs"));
}

#[test]
fn ok() {
    let t = TestCases::new();
    #[cfg(has_proc_macro_diagnostic)]
    t.pass(examples("pass_*.rs"));
}
