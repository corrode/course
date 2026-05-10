//! # Hello, Rust!
//!
//! As is common for new programming languages, we'll start with a simple
//! "Hello, World!" program. [This tradition goes back to the 1970s][hello] with
//! Brian Kernighan's C programming tutorial, and we'll follow it here.
//!
//! [hello]: https://www.hackerrank.com/blog/the-history-of-hello-world/

/// Your first exercise, how exciting!
///
/// Your task is to return the string `"Welcome, {name}!"` from this
/// function. The intro mentions `println!`, but `println!` prints. It
/// returns `()`. The macro that builds a `String` for you to return is
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html), which uses
/// the same `{name}` placeholder syntax.
fn format_welcome_message(name: &str) -> String {
    todo!("replace this line with your code")
}

// Tests live right next to the code they exercise. Don't worry about the syntax yet.
#[test]
fn test_format_welcome_message() {
    assert_eq!(format_welcome_message("Alice"), "Welcome, Alice!");
    assert_eq!(format_welcome_message("Bob"), "Welcome, Bob!");
}
