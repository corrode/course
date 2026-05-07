//! # Hello Rust!
//!
//! Welcome to the Rust programming language! 🦀
//!
//! You've just entered a world where memory safety meets performance, where
//! the compiler is your friend (even when it seems grumpy), and where
//! "fighting the borrow checker" becomes a badge of honor.
//!
//! This tradition of starting with "Hello, World!" goes back to Brian Kernighan's
//! C programming tutorial from the 1970s. But here's the thing: Rust makes sure
//! your "Hello, World!" won't accidentally corrupt memory or crash at runtime!
//!
//! Ready to make friends with the compiler? Let's go! 🚀
//!
//! Oh by the way, each lesson contains bonus exercises for curious learners.
//! These are optional and can be skipped if you're short on time.
//! You can run the bonus tests in this lesson with:
//!
//! ```sh
//! cargo test --example 00_hello_rust --features bonus
//! ```

/// Return a simple greeting message.
/// Your first step into the Rust universe!
/// Expected output: "Hello, Rust!"
fn say_hello() -> &'static str {
    todo!()
}

/// Create a personalized welcome message for a new user.
/// Expected output: "Welcome to Rust, Alice!" if name is "Alice"
fn welcome_user(name: &str) -> String {
    todo!()
}

/// BONUS: Create a formatted greeting with emoji
///
/// Rust is totally okay with emojis! They are just Unicode characters and all strings
/// in Rust are UTF-8 encoded (which is a way to represent Unicode characters).
///
/// Expected output: "{emoji} Hello there, {name}! {emoji}" (e.g., "🦀 Hello there, Rust! 🦀")
fn fancy_greeting(name: &str, emoji: &str) -> String {
    todo!()
}

/// BONUS: Generate a greeting in different languages
/// Internationalization is crucial for global apps!
/// Expected outputs:
/// "english" -> "Hello!"
/// "spanish" -> "¡Hola!"
/// "french" -> "Bonjour!"
/// others -> "Hello!
fn multilingual_hello(language: &str) -> &'static str {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_greeting() {
        assert_eq!(say_hello(), "Hello, Rust!");
    }

    #[test]
    fn test_personalized_welcome() {
        assert_eq!(welcome_user("Alice"), "Welcome to Rust, Alice!");
        assert_eq!(welcome_user("Bob"), "Welcome to Rust, Bob!");
        assert_eq!(welcome_user(""), "Welcome to Rust, !");
    }
}

#[cfg(all(test, feature = "bonus"))]
mod bonus {
    use super::*;

    #[test]
    fn test_fancy_greeting() {
        assert_eq!(fancy_greeting("Rust", "🦀"), "🦀 Hello there, Rust! 🦀");
        assert_eq!(fancy_greeting("World", "🌍"), "🌍 Hello there, World! 🌍");
    }

    #[test]
    fn test_multilingual() {
        assert_eq!(multilingual_hello("english"), "Hello!");
        assert_eq!(multilingual_hello("spanish"), "¡Hola!");
        assert_eq!(multilingual_hello("french"), "Bonjour!");
        assert_eq!(multilingual_hello("unknown"), "Hello!"); // fallback
    }
}
