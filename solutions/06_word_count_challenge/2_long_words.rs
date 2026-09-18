/// Counts whitespace-separated words containing more than three Unicode scalar values.
fn count_long_words(text: &str) -> usize {
    let mut count = 0;
    for word in text.split_whitespace() {
        if word.chars().count() > 3 {
            count += 1;
        }
    }
    count
}

#[test]
fn test_empty_and_short_words() {
    assert_eq!(count_long_words(""), 0);
    assert_eq!(count_long_words(" \t\n"), 0);
    assert_eq!(count_long_words("a to cat"), 0);
}

#[test]
fn test_long_words_and_whitespace() {
    assert_eq!(count_long_words("one  four\tfive\nseven"), 3);
}

#[test]
fn test_unicode_scalar_values() {
    assert_eq!(count_long_words("été café 猫猫猫 猫猫猫猫"), 2);
    // The combining accent is a separate scalar value: a, b, e, accent.
    assert_eq!(count_long_words("abe\u{301}"), 1);
}
