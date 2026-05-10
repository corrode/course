//! # Option<T> - Handling Missing Values
//!
//! Tony Hoare called the invention of null references in 1965 his
//! "billion-dollar mistake." He said it led to "innumerable errors,
//! vulnerabilities, and system crashes" over the years.
//!
//! Rust takes a different approach. Instead of nullable pointers that can
//! crash your program, Rust uses `Option<T>` to explicitly represent "maybe
//! something, maybe nothing." The compiler will not let you forget to check.
//!
//! The four exercises below build on each other:
//!   1. Consume an `Option` with a fallback value.
//!   2. Consume an `Option` and transform what's inside.
//!   3. Produce an `Option` from a collection.
//!   4. Produce an `Option` by searching a collection.

/// Returns the value if `Some`, otherwise returns the default.
///
/// Start here. The simplest `Option` pattern: you're handed one, and you
/// either use what's inside or fall back to a default. `match` works,
/// and `Option` also has a few helper methods that are shorter.
/// See: <https://doc.rust-lang.org/std/option/enum.Option.html>
fn get_setting_or_default(setting: Option<u32>, default: u32) -> u32 {
    todo!()
}

#[test]
fn test_setting_default() {
    assert_eq!(get_setting_or_default(Some(42), 100), 42);
    assert_eq!(get_setting_or_default(None, 100), 100);
}

/// Returns the length if `Some`, 0 if `None`.
///
/// Same idea as above, but now the fallback isn't the value itself — you
/// need to call `.len()` on the inner string first. A `match` makes both
/// branches explicit; iterator-style methods on `Option` are tidier once
/// you spot them.
/// See: <https://doc.rust-lang.org/std/keyword.match.html>
fn optional_string_length(maybe_string: Option<&str>) -> usize {
    todo!()
}

#[test]
fn test_optional_length() {
    assert_eq!(optional_string_length(Some("hello")), 5);
    assert_eq!(optional_string_length(None), 0);
}

/// Returns `Some(item)` if the list has items, `None` if empty.
///
/// Now you have to produce an `Option` instead of consume one. You
/// could write the empty-check yourself with `if items.is_empty()`,
/// but slices already have a method that returns exactly this shape.
/// See: <https://doc.rust-lang.org/std/primitive.slice.html>
fn get_first_item(items: &[String]) -> Option<&String> {
    todo!()
}

#[test]
fn test_first_item() {
    let items = vec!["first".to_string(), "second".to_string()];
    assert_eq!(get_first_item(&items), Some(&"first".to_string()));

    let empty: Vec<String> = vec![];
    assert_eq!(get_first_item(&empty), None);
}

/// Finds a user by ID in the database.
/// Returns `Some(username)` if found, `None` if not found.
///
/// The trickiest of the four: produce an `Option` by searching. The
/// iterator chapter is still ahead, but `slice::iter()` plus a search
/// combinator already gets you most of the way; the matched tuple still
/// needs to be reduced down to just the username.
fn find_user_by_id(users: &[(u32, String)], id: u32) -> Option<&str> {
    todo!()
}

#[test]
fn test_find_user() {
    let users = [
        (1, "alice".to_string()),
        (2, "bob".to_string()),
        (3, "charlie".to_string()),
    ];
    assert_eq!(find_user_by_id(&users, 2), Some("bob"));
    assert_eq!(find_user_by_id(&users, 99), None);
}
