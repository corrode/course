//! # Tuples and Destructuring
//!
//! Tuples are like gift boxes that can hold different types of treasures!
//! This concept comes from mathematics, where tuples represent ordered lists
//! of elements. René Descartes used coordinate pairs (tuples!) to invent
//! analytic geometry - the foundation of computer graphics.
//!
//! In programming, tuples are perfect when you need to return multiple values
//! from a function, or group related but different data together. Python
//! developers will feel right at home here!
//!
//! Let's unwrap some tuples and discover their secrets! 📦

/// Returns a user's name and age as a tuple.
/// Useful for functions that need to return multiple values.
fn get_user_info() -> (String, u32) {
    // Return ("Alice", 25)
    todo!()
}

/// Calculates both area and perimeter of a rectangle.
/// Returns (area, perimeter) as a tuple.
fn rectangle_measurements(width: u32, height: u32) -> (u32, u32) {
    todo!()
}

/// Extracts the first and last names from a full name tuple.
/// Takes a tuple (first_name, last_name) and returns just the first name.
fn get_first_name(full_name: (String, String)) -> String {
    // Use tuple destructuring: let (first, _last) = full_name;
    todo!()
}

/// Swaps two values using tuple destructuring.
/// Returns (second, first) - the values swapped.
fn swap_values(pair: (i32, i32)) -> (i32, i32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_info() {
        let (name, age) = get_user_info();
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
    }
    
    #[test]
    fn test_rectangle() {
        let (area, perimeter) = rectangle_measurements(5, 3);
        assert_eq!(area, 15);  // 5 * 3
        assert_eq!(perimeter, 16);  // 2 * (5 + 3)
    }
    
    #[test]
    fn test_name_extraction() {
        let full_name = ("John".to_string(), "Doe".to_string());
        assert_eq!(get_first_name(full_name), "John");
    }
    
    #[test]
    fn test_swap() {
        assert_eq!(swap_values((1, 2)), (2, 1));
        assert_eq!(swap_values((42, 100)), (100, 42));
    }
}