//! # Hello, Rust!
//!
//! As is common for new programming languages, we'll start with a simple
//! "Hello, World!" program. [This tradition goes back to the 1970s][hello] with
//! Brian Kernighan's C programming tutorial, and we'll follow it here.
//!
//! [hello]: https://www.hackerrank.com/blog/the-history-of-hello-world/

/// Your first exercise, how exciting!
///
/// Your task is to format return "Welcome, {name}!"
fn format_welcome_message(name: &str) -> String {
    todo!("replace this line with your code")
}

// Oh, note! This part below is just for testing. You don't need to read or
// understand it yet. But also: this is just Rust code, so if you're curious you can!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_message() {
        assert_eq!(format_welcome_message("Alice"), "Welcome, Alice!");
        assert_eq!(format_welcome_message("Bob"), "Welcome, Bob!");
    }
}
