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
fn find_user_by_id<'a>(users: &'a [(u32, &'a str)], id: u32) -> Option<&'a str> {
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

    // Bonus exercises - run with: cargo test --example 06_option_and_some_none --features bonus
    #[cfg(feature = "bonus")]
    mod bonus {
        use super::*;

        /// BONUS: Chain multiple optional operations together
        /// Data pipeline systems often chain transformations like this!
        fn chain_operations(value: Option<i32>) -> Option<i32> {
            todo!()
        }

        /// BONUS: Find the maximum value in a list of optional numbers
        /// Analytics systems need to handle missing data gracefully!
        fn max_of_options(numbers: &[Option<i32>]) -> Option<i32> {
            todo!()
        }

        /// BONUS: Parse and validate an optional configuration string
        /// Configuration systems need robust parsing with defaults!
        fn parse_port_config(config: Option<&str>) -> Option<u16> {
            todo!()
        }

        /// BONUS: Flatten nested options
        /// Network programming often has nested optional results!
        fn flatten_option(nested: Option<Option<String>>) -> Option<String> {
            todo!()
        }

        #[test]
        fn test_chaining() {
            // Should: multiply by 2, add 10, then return if even
            assert_eq!(chain_operations(Some(5)), Some(20)); // 5*2+10=20 (even)
            assert_eq!(chain_operations(Some(4)), None); // 4*2+10=18 (even, but let's say we want odd results)
            assert_eq!(chain_operations(None), None);
        }

        #[test]
        fn test_max_options() {
            let numbers = [Some(5), None, Some(10), Some(2)];
            assert_eq!(max_of_options(&numbers), Some(10));

            let all_none = [None, None, None];
            assert_eq!(max_of_options(&all_none), None);
        }

        #[test]
        fn test_port_parsing() {
            assert_eq!(parse_port_config(Some("8080")), Some(8080));
            assert_eq!(parse_port_config(Some("invalid")), None);
            assert_eq!(parse_port_config(None), None);
        }

        #[test]
        fn test_flattening() {
            assert_eq!(
                flatten_option(Some(Some("hello".to_string()))),
                Some("hello".to_string())
            );
            assert_eq!(flatten_option(Some(None)), None);
            assert_eq!(flatten_option(None), None);
        }
    }
}
