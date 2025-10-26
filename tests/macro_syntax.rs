#[test]
fn check_derive_macro_syntax() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive_macro_syntax/*.rs");
    t.pass("tests/derive_macro_expansion/*.rs");
}
