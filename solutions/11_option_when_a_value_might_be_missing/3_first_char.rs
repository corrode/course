/// Returns the first character of `text`, or `None` if the string is empty.
fn first_char(text: &str) -> Option<char> {
    text.chars().next()
}

#[test]
fn test_first_char() {
    assert_eq!(first_char("hello"), Some('h'));
    assert_eq!(first_char("rust"), Some('r'));
    assert_eq!(first_char(""), None);
}
