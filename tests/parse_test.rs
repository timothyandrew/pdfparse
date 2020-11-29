#[test]
fn test_basic() {
    let html = std::fs::read_to_string("tests/assert/basic.html").unwrap();
    let md = std::fs::read_to_string("tests/assert/basic.md").unwrap();
    assert_eq!(pdfparse::parse_html(&html), md);
}
