#[test]
fn test() {
    let t = trybuild::TestCases::new();
    // builder
    t.pass("tests/builder/struct.rs");
    t.pass("tests/builder/prefix.rs");
    t.pass("tests/builder/rename.rs");
    t.compile_fail("tests/builder/skip.rs");
    t.compile_fail("tests/builder/no_into.rs");
    t.compile_fail("tests/builder/enum.rs");
    t.compile_fail("tests/builder/newtype.rs");
    t.compile_fail("tests/builder/tuple.rs");
    t.compile_fail("tests/builder/unit.rs");
    t.compile_fail("tests/builder/bad_attr.rs");
    t.compile_fail("tests/builder/prefix_non_string.rs");
    t.compile_fail("tests/builder/rename_non_string.rs");
    // from
    t.pass("tests/from/enum.rs");
    t.pass("tests/from/struct.rs");
    t.pass("tests/from/newtype.rs");
    t.pass("tests/from/unit.rs");
    t.pass("tests/from/tuple.rs");
    t.compile_fail("tests/from/bad_attr.rs");
    t.compile_fail("tests/from/skip.rs");
}
