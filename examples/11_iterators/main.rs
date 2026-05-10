//! # Iterators
//!
//! Iterators were popularised by functional languages like Lisp (created by
//! John McCarthy in 1958), and today they're a core building block in most
//! modern languages.
//!
//! Rust's iterators are lazy: they don't do any work until you ask for a
//! result. You can chain operations together like a pipeline, and the
//! compiler will usually fuse them into a single tight loop.
//!
//! The four exercises below introduce one new combinator at a time:
//!   1. `sum`        : collapse an iterator down to a single value.
//!   2. `map`        : transform every element.
//!   3. `filter`     : keep only the elements you want.
//!   4. `filter` + `to_string`: same, but with a borrowing twist.
//!
//! Heads-up on closure arguments: `.iter()` yields `&T`, but `filter`
//! gives its closure another reference on top, so the closure sees
//! `&&T`. That's why you'll often see `**c == ...` or `s.starts_with(...)`
//! (which auto-derefs) instead of plain `c == ...`. Don't be alarmed
//! when the compiler complains about a missing `&`, see
//! [the cheatsheet](/cheatsheet) entry on iterators.

/// Calculates total revenue from sales data.
///
/// The simplest iterator pattern: take a sequence, produce one number.
/// You could write a `for` loop with a running total, but the standard
/// library can collapse a numeric iterator down for you in one call.
/// See: <https://doc.rust-lang.org/std/iter/trait.Iterator.html>
fn calculate_total_revenue() -> i32 {
    let sales = vec![1200, 850, 2300, 950, 1800, 3200, 1100, 2800];
    todo!()
}

#[test]
fn test_revenue_calculation() {
    let total = calculate_total_revenue();
    assert_eq!(total, 14200); // Sum of all sales
}

/// Normalizes email addresses to lowercase.
///
/// Now you need to transform every element instead of collapsing the
/// sequence. The shape is `vec.into_iter()` -> some combinator that
/// applies a closure -> back to a `Vec` via `collect()`.
/// See: <https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase>
fn normalize_emails(emails: Vec<String>) -> Vec<String> {
    todo!()
}

#[test]
fn test_email_normalization() {
    let emails = vec!["Alice@EXAMPLE.COM".to_string(), "BOB@test.ORG".to_string()];
    let normalized = normalize_emails(emails);
    assert_eq!(normalized, vec!["alice@example.com", "bob@test.org"]);
}

/// Returns all users whose usernames start with 'a'.
///
/// Same idea, but instead of transforming each element you keep some
/// and drop others. Watch out for one borrowing gotcha: the closure
/// receives a reference to each element, not the element itself.
/// See: <https://doc.rust-lang.org/std/primitive.str.html#method.starts_with>
fn select_usernames_starting_with_a(usernames: Vec<&str>) -> Vec<&str> {
    todo!()
}

#[test]
fn test_active_users() {
    let users = vec!["alice", "admin", "bob", "anonymous", "charlie"];
    let active = select_usernames_starting_with_a(users);
    assert_eq!(active, vec!["alice", "admin", "anonymous"]);
}

/// Finds all files with ".rs" extension.
///
/// Same shape as the previous one, but the input is a `&[&str]` (a
/// borrowed slice of borrowed strings), so the iterator yields `&&str`.
/// We sidestep that double-reference by returning owned `String`s; the
/// lesson here is iterators, not lifetimes. To go from `&&str` to
/// `String`, reach for [`str::to_string`].
fn find_rust_files(files: &[&str]) -> Vec<String> {
    todo!()
}

#[test]
fn test_rust_files() {
    let files = &[
        "main.rs",
        "README.md",
        "lib.rs",
        "package.json",
        "config.rs",
    ];
    let rust_files = find_rust_files(files);
    assert_eq!(rust_files, vec!["main.rs", "lib.rs", "config.rs"]);
}
