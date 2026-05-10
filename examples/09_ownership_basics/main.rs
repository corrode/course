//! # Ownership Basics
//!
//! Ownership is the feature that makes Rust different from most other
//! languages. C and C++ make you manage memory manually, which leads to a
//! whole category of bugs: use-after-free, double-free, dangling pointers.
//! Garbage-collected languages avoid those bugs at the cost of unpredictable
//! pauses and runtime overhead.
//!
//! Rust takes a third path. The compiler tracks who owns each value, and
//! when an owner goes out of scope the value is dropped. The rules are
//! enforced at compile time, so there's no garbage collector at runtime.
//!
//! The ownership system was inspired by ideas from academia, particularly
//! linear and affine types. You don't need to know the theory; you'll build
//! intuition by writing code.

/// Takes ownership of a String and modifies it.
/// When you pass a String to this function, ownership transfers.
fn take_ownership(s: String) -> String {
    // Add " - owned by Rust!" to the end and return
    todo!()
}

#[test]
fn test_ownership() {
    let s = String::from("Rust");
    let result = take_ownership(s);
    // Note: s is no longer valid here! It was moved.
    assert_eq!(result, "Rust - owned by Rust!");
}

/// Borrows a string reference without taking ownership.
/// The original string remains valid after this function returns.
fn borrow_string(s: &str) -> usize {
    // Return the length of the string
    todo!()
}

#[test]
fn test_borrowing() {
    let s = "The Matrix has you";
    let len = borrow_string(s);
    // s is still valid here because we only borrowed it
    assert_eq!(len, 18);
    assert_eq!(s, "The Matrix has you"); // Still here!
}

/// Takes a mutable reference to modify the string in place.
/// The &mut allows us to change the string's contents.
fn mutate_string(s: &mut String) {
    // Append " - now with extra crab" to the string
    todo!()
}

#[test]
fn test_mutable_borrow() {
    let mut s = String::from("Ferris");
    mutate_string(&mut s);
    assert_eq!(s, "Ferris - now with extra crab");
}
