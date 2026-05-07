//! # Strings and Text Processing
//!
//! Text is everywhere in software! From user names to search queries, 
//! from log messages to configuration files. In fact, you're reading 
//! text right now! 📝
//!
//! Here's something cool: Rust's String type guarantees valid UTF-8 
//! encoding. This means emoji, accented characters, and text in any 
//! language work perfectly. No more "mystery characters" or encoding bugs!
//!
//! Fun story: The difference between String and &str trips up many 
//! newcomers, but think of it like this - String is like owning a book, 
//! &str is like borrowing a page from someone else's book.
//!
//!## Bonus Exercises
//!
//! This lesson contains bonus exercises for curious learners.
//! These are optional and can be skipped if you're short on time.
//! You can run the bonus tests in this lesson with:
//!
//! ```sh
//! cargo test --example 03_strings_and_text --features bonus
//! ```

/// Format a user's full name for display
/// Social media apps do this constantly
/// Expected output: "{first} {last}" (e.g., "Alice Johnson")
fn format_full_name(first: &str, last: &str) -> String {
    todo!()
}

/// Get the length of a user's input
/// Form validation checks input length
/// Expected output: number of bytes in the string (e.g., "hello" -> 5)
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.len
fn get_text_length(text: &str) -> usize {
    todo!()
}

/// Convert text to uppercase for standardization
/// Database systems normalize text this way
/// Expected output: all letters converted to uppercase (e.g., "hello" -> "HELLO")
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase
fn to_uppercase(text: &str) -> String {
    todo!()
}

/// Check if text is empty (contains no characters)
/// Input validation needs to check for empty fields
/// Expected output: true if text has 0 characters, false otherwise
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
fn is_empty_text(text: &str) -> bool {
    todo!()
}

/// Create a greeting message with a name
/// Chat applications and email systems do this constantly
/// Expected output: "Hello, {name}!" (e.g., "Hello, Alice!")
/// See: https://doc.rust-lang.org/std/macro.format.html
fn create_greeting(name: &str) -> String {
    todo!()
}

/// BONUS: Repeat a string multiple times
/// Template systems and text generation use string repetition
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.repeat
fn repeat_string(text: &str, times: usize) -> String {
    todo!()
}

/// BONUS: Check if text starts with a specific prefix
/// URL routing and file systems use prefix checking
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.starts_with
fn starts_with_prefix(text: &str, prefix: &str) -> bool {
    todo!()
}

/// BONUS: Truncate text to a maximum length
/// Social media platforms limit message length this way
/// See: https://doc.rust-lang.org/std/primitive.str.html (string slicing)
fn truncate_text(text: &str, max_length: usize) -> &str {
    todo!()
}

/// BONUS: Replace all occurrences of a character
/// Text processing systems need character replacement
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.replace
fn replace_character(text: &str, old_char: char, new_char: char) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_formatting() {
        assert_eq!(format_full_name("Alice", "Johnson"), "Alice Johnson");
        assert_eq!(format_full_name("", "Smith"), " Smith");
        assert_eq!(format_full_name("Bob", ""), "Bob ");
    }

    #[test]
    fn test_text_length() {
        assert_eq!(get_text_length("hello"), 5);
        assert_eq!(get_text_length(""), 0);
        assert_eq!(get_text_length("🦀"), 4); // Emoji takes 4 bytes in UTF-8!
    }

    #[test]
    fn test_uppercase_conversion() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_uppercase("Rust"), "RUST");
        assert_eq!(to_uppercase(""), "");
    }

    #[test]
    fn test_empty_check() {
        assert_eq!(is_empty_text(""), true);
        assert_eq!(is_empty_text("hello"), false);
        assert_eq!(is_empty_text(" "), false); // Space is not empty!
    }

    #[test]
    fn test_greeting_creation() {
        assert_eq!(create_greeting("Alice"), "Hello, Alice!");
        assert_eq!(create_greeting("Bob"), "Hello, Bob!");
        assert_eq!(create_greeting(""), "Hello, !"); // Works with empty names too
    }
}

#[cfg(all(test, feature = "bonus"))]
mod bonus {
    use super::*;

    #[test]
    fn test_string_repetition() {
        assert_eq!(repeat_string("hi", 3), "hihihi");
        assert_eq!(repeat_string("x", 0), "");
        assert_eq!(repeat_string("", 5), "");
    }

    #[test]
    fn test_prefix_checking() {
        assert_eq!(starts_with_prefix("hello world", "hello"), true);
        assert_eq!(starts_with_prefix("rust programming", "python"), false);
        assert_eq!(starts_with_prefix("test", ""), true); // Empty prefix always matches
    }

    #[test]
    fn test_text_truncation() {
        assert_eq!(truncate_text("hello world", 5), "hello");
        assert_eq!(truncate_text("short", 10), "short");
        assert_eq!(truncate_text("test", 0), "");
    }

    #[test]
    fn test_character_replacement() {
        assert_eq!(replace_character("hello", 'l', 'x'), "hexxo");
        assert_eq!(replace_character("rust", 'z', 'x'), "rust"); // No change
        assert_eq!(replace_character("", 'a', 'b'), "");
    }
}