/// Returns the length if `Some`, 0 if `None`.
fn optional_string_length(maybe_string: Option<&str>) -> usize {
    maybe_string.map_or(0, |s| s.len())
}

#[test]
fn test_optional_string_length() {
    assert_eq!(optional_string_length(Some("hello")), 5);
    assert_eq!(optional_string_length(None), 0);
}
