fn optional_string_length(maybe_string: Option<&str>) -> usize {
    todo!("Return the string length in bytes for Some, or 0 for None")
}

#[test]
fn test_optional_string_length() {
    assert_eq!(optional_string_length(Some("hello")), 5);
    assert_eq!(optional_string_length(None), 0);
}
