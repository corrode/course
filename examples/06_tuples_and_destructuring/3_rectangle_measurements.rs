//! # Computing two values at once
//!
//! When two results are naturally produced together, returning them as a
//! tuple is often clearer than two separate function calls. The caller
//! destructures the result into named bindings.

/// Calculates both area and perimeter of a rectangle.
/// Returns (area, perimeter) as a tuple.
fn rectangle_measurements(width: u32, height: u32) -> (u32, u32) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_measurements() {
        let (area, perimeter) = rectangle_measurements(5, 3);
        assert_eq!(area, 15); // 5 * 3
        assert_eq!(perimeter, 16); // 2 * (5 + 3)
    }
}
