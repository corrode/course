//! # Option<T> - Handling Missing Values
//!
//! The billion-dollar mistake! That's what Tony Hoare called his invention of
//! null references in 1965. He said it led to "innumerable errors, vulnerabilities,
//! and system crashes" over the years.
//!
//! Rust learned from this history and chose a different path. Instead of null
//! pointers that can crash your program, Rust uses Option<T> to explicitly
//! represent "maybe something, maybe nothing." It's impossible to forget to
//! check - the compiler won't let you!
//!
//! Welcome to the future of safe programming! 🛡️

/// Finds a user by ID in the database.
/// Returns Some(username) if found, None if not found.
///
/// See: https://doc.rust-lang.org/std/primitive.slice.html#method.iter
/// See: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find
/// See: https://doc.rust-lang.org/std/option/enum.Option.html#method.map
fn find_user_by_id(users: &[(u32, &str)], id: u32) -> Option<&str> {
    todo!()
}

/// Gets the first item from a list.
/// Returns Some(item) if list has items, None if empty.
fn get_first_item(items: &[String]) -> Option<&String> {
    todo!()
}

/// Gets a configuration value with a default fallback.
/// Returns the value if Some, otherwise returns the default.
fn get_setting_or_default(setting: Option<u32>, default: u32) -> u32 {
    todo!()
}

/// Safely gets the length of an optional string.
/// Returns the length if Some, 0 if None.
/// See: https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or
/// See: https://doc.rust-lang.org/std/keyword.match.html
fn optional_string_length(maybe_string: Option<&str>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_user() {
        let users = [(1, "alice"), (2, "bob"), (3, "charlie")];
        assert_eq!(find_user_by_id(&users, 2), Some("bob"));
        assert_eq!(find_user_by_id(&users, 99), None);
    }

    #[test]
    fn test_first_item() {
        let items = vec!["first".to_string(), "second".to_string()];
        assert_eq!(get_first_item(&items), Some(&"first".to_string()));

        let empty: Vec<String> = vec![];
        assert_eq!(get_first_item(&empty), None);
    }

    #[test]
    fn test_setting_default() {
        assert_eq!(get_setting_or_default(Some(42), 100), 42);
        assert_eq!(get_setting_or_default(None, 100), 100);
    }

    #[test]
    fn test_optional_length() {
        assert_eq!(optional_string_length(Some("hello")), 5);
        assert_eq!(optional_string_length(None), 0);
    }
}
