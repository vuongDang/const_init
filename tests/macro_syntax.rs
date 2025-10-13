#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive_macro_syntax/*.rs");
    t.pass("tests/derive_macro_expansion/*.rs");
}
