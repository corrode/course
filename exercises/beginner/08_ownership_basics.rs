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
    unimplemented!()
}

/// Borrows a string reference without taking ownership.
/// The original string remains valid after this function returns.
fn borrow_string(s: &str) -> usize {
    // Return the length using .len()
    unimplemented!()
}

/// Takes a mutable reference to modify the string in place.
/// The &mut allows us to change the string's contents.
fn mutate_string(s: &mut String) {
    // Add " - now with 100% more crab!" using .push_str()
    unimplemented!()
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
}