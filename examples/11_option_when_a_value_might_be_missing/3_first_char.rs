fn first_char(text: &str) -> Option<char> {
    todo!("Return the first Unicode character of text, or None if empty")
}

#[test]
fn test_first_char() {
    assert_eq!(first_char("hello"), Some('h'));
    assert_eq!(first_char("rust"), Some('r'));
    assert_eq!(first_char(""), None);
}
