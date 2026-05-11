//! # Making a function `pub`
//!
//! Modules organise code and control what's accessible. By default,
//! everything inside a module is **private** to that module. The `pub`
//! keyword lifts the gate item-by-item.
//!
//! This step is deliberately broken. Run `cargo check --example
//! 14_modules_and_visibility` (or just hit Run in the editor) and read
//! the compiler error: it's telling you exactly which item needs to be
//! made `pub`. Fix the smallest thing that makes it compile.

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
