//! # Your first function
//!
//! Your task is to return the string `"Welcome, {name}!"` from this
//! function. The intro mentions `println!`, but `println!` *prints*;
//! it returns `()`. The macro that builds a `String` for you to
//! return is [`format!`](https://doc.rust-lang.org/std/macro.format.html),
//! which uses the same `{name}` placeholder syntax.

/// Your first exercise, how exciting!
fn format_welcome_message(name: &str) -> String {
    todo!("replace this line with your code")
}

// Tests live right next to the code they exercise. Don't worry about the syntax yet.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_welcome_message() {
        assert_eq!(format_welcome_message("Alice"), "Welcome, Alice!");
        assert_eq!(format_welcome_message("Bob"), "Welcome, Bob!");
    }
}
