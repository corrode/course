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
/// Expected output: ("Alice".to_string(), 25)
/// Useful for functions that need to return multiple values.
fn get_user_info() -> (String, u32) {
    todo!()
}

/// Calculates both area and perimeter of a rectangle.
/// Expected output: (area, perimeter) where area = width * height, perimeter = 2 * (width + height)
fn rectangle_measurements(width: u32, height: u32) -> (u32, u32) {
    todo!()
}

/// Extracts the first and last names from a full name tuple.
/// Expected output: returns just the first name from the tuple
/// Hint: Use [tuple destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
fn get_first_name(full_name: (String, String)) -> String {
    todo!()
}

/// Swaps two values using tuple destructuring.
/// Expected output: (second, first) - the values swapped
/// See: https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
fn swap_values(pair: (i32, i32)) -> (i32, i32) {
    todo!()
}

// Bonus exercises for curious learners!
// Run with: cargo test --example 05_tuples_and_structs --features bonus
#[cfg(feature = "bonus")]
mod bonus {
    /// BONUS: Calculate 3D distance between two points
    /// 3D graphics engines use this constantly for collision detection!
    /// See: https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
    pub fn distance_3d(point1: (f64, f64, f64), point2: (f64, f64, f64)) -> f64 {
        todo!()
    }

    /// BONUS: Destructure a nested tuple and calculate its sum
    /// Financial software often processes nested data structures!
    pub fn sum_nested_tuple(data: ((i32, i32), (i32, i32))) -> i32 {
        todo!()
    }

    /// BONUS: Convert RGB tuple to HSV color space
    /// Image processing software converts between color spaces!
    /// See: https://doc.rust-lang.org/std/primitive.f64.html (for min/max)
    pub fn rgb_to_hue(rgb: (u8, u8, u8)) -> f64 {
        todo!()
    }
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
        assert_eq!(area, 15); // 5 * 3
        assert_eq!(perimeter, 16); // 2 * (5 + 3)
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

    #[cfg(feature = "bonus")]
    mod bonus_tests {
        use super::super::bonus::*;

        #[test]
        fn test_3d_distance() {
            let p1 = (0.0, 0.0, 0.0);
            let p2 = (3.0, 4.0, 0.0);
            assert!((distance_3d(p1, p2) - 5.0).abs() < 0.01);
        }

        #[test]
        fn test_nested_sum() {
            let data = ((1, 2), (3, 4));
            assert_eq!(sum_nested_tuple(data), 10);
        }

        #[test]
        fn test_rgb_conversion() {
            let red = (255, 0, 0);
            assert!((rgb_to_hue(red) - 0.0).abs() < 0.01);

            let green = (0, 255, 0);
            assert!((rgb_to_hue(green) - 120.0).abs() < 0.01);
        }
    }
}
