//! # Tuples and Destructuring
//!
//! A tuple is a fixed-size group of values, possibly of different types.
//! The concept comes from mathematics, where tuples are ordered lists of
//! elements. René Descartes used coordinate pairs (which are tuples) to
//! invent analytic geometry, the foundation of computer graphics.
//!
//! In Rust, tuples are useful when a function needs to return more than one
//! value, or when you want to group a few related values without defining a
//! whole new struct.

/// Returns a user's name and age as a tuple.
/// For example, return "Alice" and 25.
/// Useful for functions that need to return multiple values.
fn get_user_info() -> (String, u32) {
    todo!()
}

#[test]
fn test_get_user_info() {
    let (name, age) = get_user_info();
    assert_eq!(name, "Alice");
    assert_eq!(age, 25);
}

/// Calculates both area and perimeter of a rectangle.
/// Returns (area, perimeter) as a tuple.
fn rectangle_measurements(width: u32, height: u32) -> (u32, u32) {
    todo!()
}

#[test]
fn test_rectangle_measurements() {
    let (area, perimeter) = rectangle_measurements(5, 3);
    assert_eq!(area, 15); // 5 * 3
    assert_eq!(perimeter, 16); // 2 * (5 + 3)
}

/// Extracts the first and last names from a full name tuple.
/// Takes a tuple (first_name, last_name) and returns just the first name.
///
/// Note: this signature moves the tuple in (it contains `String`s, which
/// aren't `Copy`). After the call, the original `full_name` is no longer
/// valid; try using it after calling this function and read the error.
/// `swap_values` below takes `(i32, i32)`, which is `Copy`, so the
/// original is still usable. Chapter 9 covers this properly.
///
/// Hint: Use [tuple destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
fn get_first_name(full_name: (String, String)) -> String {
    todo!()
}

#[test]
fn test_get_first_name() {
    let full_name = ("John".to_string(), "Doe".to_string());
    assert_eq!(get_first_name(full_name), "John");
}

/// Swaps two values using tuple destructuring.
fn swap_values(pair: (i32, i32)) -> (i32, i32) {
    todo!()
}

#[test]
fn test_swap_values() {
    assert_eq!(swap_values((1, 2)), (2, 1));
    assert_eq!(swap_values((42, 100)), (100, 42));
}
