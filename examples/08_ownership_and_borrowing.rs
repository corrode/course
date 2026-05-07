//! # Ownership Basics
//!
//! Welcome to Rust's superpower! Ownership is what makes Rust special.
//! While other languages either make you manage memory manually (hello,
//! segfaults!) or use garbage collection (goodbye, predictable performance!),
//! Rust found a third way.
//!
//! The ownership system was inspired by ideas from academia, particularly
//! linear types and affine types. But don't worry about the theory -
//! you'll build intuition through practice! Think of it as Rust being
//! a very organized friend who always knows who's borrowing what.
//!
//! Once you master ownership, you'll wonder how you ever lived without it! 🦀

/// Takes ownership of a String and modifies it.
/// When you pass a String to this function, ownership transfers.
fn take_ownership(s: String) -> String {
    // Add " - owned by Rust!" to the end and return
    todo!()
}

/// Borrows a string reference without taking ownership.
/// The original string remains valid after this function returns.
fn borrow_string(s: &str) -> usize {
    // Return the length of the string
    todo!()
}

/// Takes a mutable reference to modify the string in place.
/// The &mut allows us to change the string's contents.
fn mutate_string(s: &mut String) {
    // Append " - now with 100% more crab!" to the string
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership() {
        let s = String::from("Rust");
        let result = take_ownership(s);
        // Note: s is no longer valid here! It was moved.
        assert_eq!(result, "Rust - owned by Rust!");
    }

    #[test]
    fn test_borrowing() {
        let s = "The Matrix has you";
        let len = borrow_string(s);
        // s is still valid here because we only borrowed it
        assert_eq!(len, 18);
        assert_eq!(s, "The Matrix has you"); // Still here!
    }

    #[test]
    fn test_mutable_borrow() {
        let mut s = String::from("Ferris");
        mutate_string(&mut s);
        assert_eq!(s, "Ferris - now with 100% more crab!");
    }

    // Bonus exercises - run with: cargo test --example 08_ownership_and_borrowing --features bonus
    #[cfg(feature = "bonus")]
    mod bonus {
        use super::*;

        /// BONUS: Demonstrate the clone pattern for keeping data
        /// Sometimes you need to keep the original and create a copy!
        fn clone_and_modify(original: &str) -> (String, String) {
            todo!()
        }

        /// BONUS: Work with borrowed slices from a Vec
        /// This pattern is common in data processing systems!
        fn process_slice(numbers: &[i32]) -> (i32, i32) {
            todo!()
        }

        /// BONUS: Return a reference with proper lifetime management
        /// Advanced pattern for zero-copy string processing!
        fn find_longest_word<'a>(words: &[&'a str]) -> Option<&'a str> {
            todo!()
        }

        /// BONUS: Demonstrate move semantics with collections
        /// Understanding when Vec<T> gets moved is crucial!
        fn consume_and_transform(mut data: Vec<String>) -> Vec<String> {
            todo!()
        }

        #[test]
        fn test_cloning() {
            let (original, modified) = clone_and_modify("Hello");
            assert_eq!(original, "Hello");
            assert_eq!(modified, "Hello, World!");
        }

        #[test]
        fn test_slice_processing() {
            let numbers = vec![1, 5, 3, 9, 2];
            let (min, max) = process_slice(&numbers);
            assert_eq!(min, 1);
            assert_eq!(max, 9);
            // numbers is still valid here!
            assert_eq!(numbers.len(), 5);
        }

        #[test]
        fn test_longest_word() {
            let words = ["the", "quick", "brown", "fox"];
            assert_eq!(find_longest_word(&words), Some("quick"));

            let empty: [&str; 0] = [];
            assert_eq!(find_longest_word(&empty), None);
        }

        #[test]
        fn test_move_and_transform() {
            let data = vec!["rust".to_string(), "is".to_string(), "awesome".to_string()];
            let result = consume_and_transform(data);
            // data is no longer valid here - it was moved!
            assert_eq!(result, vec!["RUST", "IS", "AWESOME"]);
        }
    }
}
