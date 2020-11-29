fn execute_test(name: &str) {
    let html = std::fs::read_to_string(format!("tests/assert/{}.html", name)).unwrap();
    let md = std::fs::read_to_string(format!("tests/assert/{}.md", name)).unwrap();
    assert_eq!(pdfparse::parse_html(&html), md);
}

#[test]
fn test_basic() {
    execute_test("basic")
}

#[test]
fn test_intermediate() {
    execute_test("int1")
}
