// Module with a function.
mod calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// Wraps `calculator::add`. To make this compile, you need to make
/// `add` (in the `calculator` module above) public with `pub fn add`.
fn calculate(x: i32, y: i32) -> i32 {
    calculator::add(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(calculate(2, 3), 5);
    }
}
