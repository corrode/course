use std::collections::HashMap;

/// Counts how many times each word appears in the text.
/// Words are whitespace-separated; counts use lowercase keys.
fn count_words(text: &str) -> HashMap<String, usize> {
    todo!()
}

#[test]
fn test_count_words() {
    let text = "hello world hello rust world";
    let counts = count_words(text);
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&1));
}

#[test]
fn test_count_words_case_insensitive() {
    let text = "Hello HELLO hello";
    let counts = count_words(text);
    assert_eq!(counts.get("hello"), Some(&3));
}

#[test]
fn test_count_words_unicode_and_whitespace() {
    let counts = count_words("CAFÉ\tCafé\n🦀\u{2003}🦀");
    assert_eq!(counts.get("café"), Some(&2));
    assert_eq!(counts.get("🦀"), Some(&2));
    assert_eq!(counts.len(), 2);
    assert!(count_words("").is_empty());
    assert!(count_words(" \t\n").is_empty());
}
