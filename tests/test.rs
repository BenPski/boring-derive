#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/builder/example.rs")
}
