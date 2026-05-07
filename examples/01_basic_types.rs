//! # Basic Data Types
//!
//! Every programming language has its building blocks, and Rust is no different!
//! Think of data types as different containers for information - some hold
//! numbers, some hold text, some hold true/false values.
//!
//! Here's a fun fact: Rust's type system caught a critical bug in Dropbox's
//! storage system. [^dropbox] The type system really is your friend!
//!
//! Today we'll meet Rust's most fundamental types. Don't worry about
//! memorizing everything - focus on understanding the concepts!
//!
//! [^dropbox]: See https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine
//!
//!## Bonus Exercises
//!
//! This lesson contains bonus exercises for curious learners.
//! These are optional and can be skipped if you're short on time.
//! You can run the bonus tests in this lesson with:
//!
//! ```sh
//! cargo test --example 01_basic_types --features bonus
//! ```

/// Check if a user is eligible for our senior discount (65 or older)
/// This function works with unsigned integers (u8 can hold 0-255)
/// Expected output: true if age >= 65, false otherwise
fn is_senior_citizen(age: u8) -> bool {
    todo!()
}

/// Calculate how much coffee we can afford
/// Coffee shops use this logic for budget calculations
/// Expected output: number of full cups we can buy (e.g., budget=10.0, price=2.50 -> 4 cups)
fn cups_of_coffee_affordable(budget: f32, price_per_cup: f32) -> u32 {
    todo!()
}

/// Check if our inventory system is healthy
/// System monitoring uses boolean logic like this
/// Expected output: true only if BOTH database AND payment system are online
fn system_status_ok(database_online: bool, payment_system_online: bool) -> bool {
    todo!()
}

/// Get the first character of a user's name for their avatar
/// Rust's `char` type can hold any Unicode character - emojis included!
/// Expected output: first character of the name (e.g., "Alice" -> 'A')
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.chars
fn get_initial(name: &str) -> char {
    todo!()
}

/// BONUS: Convert between different number types safely
/// Type conversion is crucial in systems programming
/// See: https://doc.rust-lang.org/std/convert/trait.From.html
fn safe_u8_to_u32(value: u8) -> u32 {
    todo!()
}

/// BONUS: Calculate the absolute difference between two numbers
/// This uses only basic arithmetic - no if statements needed!
/// Financial systems use absolute differences for calculations
fn absolute_difference(a: f32, b: f32) -> f32 {
    todo!()
}

/// BONUS: Determine if a number is even using modulo operator
/// The % operator gives you the remainder after division
/// See: https://doc.rust-lang.org/book/appendix-02-operators.html
fn is_even(number: u32) -> bool {
    todo!()
}

/// BONUS: Convert temperature from Celsius to Fahrenheit
/// Weather apps use this conversion
/// Formula: F = C * 9/5 + 32
fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_age_checking() {
        assert_eq!(is_senior_citizen(65), true);
        assert_eq!(is_senior_citizen(70), true);
        assert_eq!(is_senior_citizen(64), false);
        assert_eq!(is_senior_citizen(25), false);
    }

    #[test]
    fn test_coffee_budget() {
        assert_eq!(cups_of_coffee_affordable(10.0, 2.50), 4);
        assert_eq!(cups_of_coffee_affordable(5.0, 3.00), 1);
        assert_eq!(cups_of_coffee_affordable(1.0, 3.00), 0);
    }

    #[test]
    fn test_system_health() {
        assert_eq!(system_status_ok(true, true), true);
        assert_eq!(system_status_ok(true, false), false);
        assert_eq!(system_status_ok(false, true), false);
        assert_eq!(system_status_ok(false, false), false);
    }

    #[test]
    fn test_name_initials() {
        assert_eq!(get_initial("Alice"), 'A');
        assert_eq!(get_initial("bob"), 'b');
        assert_eq!(get_initial("🦀"), '🦀'); // Rust loves Unicode!
    }
}

#[cfg(all(test, feature = "bonus"))]
mod bonus {
    use super::*;

    #[test]
    fn test_type_conversion() {
        assert_eq!(safe_u8_to_u32(42), 42);
        assert_eq!(safe_u8_to_u32(255), 255);
        assert_eq!(safe_u8_to_u32(0), 0);
    }

    #[test]
    fn test_absolute_difference() {
        assert_eq!(absolute_difference(10.0, 5.0), 5.0);
        assert_eq!(absolute_difference(5.0, 10.0), 5.0);
        assert_eq!(absolute_difference(7.5, 7.5), 0.0);
    }

    #[test]
    fn test_even_numbers() {
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(100), true);
    }

    #[test]
    fn test_temperature_conversion() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 0.1);
    }
}

