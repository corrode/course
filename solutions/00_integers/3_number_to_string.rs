/// Convert a number to a string.
fn number_to_string(number: u32) -> String {
    number.to_string()
}

#[test]
fn test_number_to_string() {
    assert_eq!(number_to_string(1234), "1234");
    assert_eq!(number_to_string(0), "0");
}
