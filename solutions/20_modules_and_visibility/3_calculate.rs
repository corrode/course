// Module with a function.
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

/// Wraps `calculator::add` so callers don't have to know which
/// module the addition lives in.
fn calculate(x: i32, y: i32) -> i32 {
    calculator::add(x, y)
}

#[test]
fn test_calculate() {
    assert_eq!(calculate(2, 3), 5);
}
