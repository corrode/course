//! # The ? Operator - Elegant Error Propagation
//!
//! The question mark operator is Rust's secret weapon for clean error handling!
//! Instead of writing verbose match statements for every Result, you can use `?`
//! to automatically propagate errors up the call stack.
//!
//! This exercise builds on Result<T, E> from the previous lesson. The key insight:
//! `?` eliminates boilerplate when you want to "bubble up" errors.

use std::fs;

/// Adds two parsed numbers. Compare this to doing it with match statements!
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.parse
fn add_parsed_numbers(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

/// Reads a file and counts lines. Notice how ? works with different error types!
/// See: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
fn count_file_lines(filename: &str) -> Result<usize, std::io::Error> {
    todo!()
}

/// Combines both parsing AND file I/O errors using Box<dyn Error>
/// This shows how ? can handle multiple error types in one function!
/// See: https://doc.rust-lang.org/std/error/trait.Error.html
fn sum_numbers_in_file(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parsing() {
        assert_eq!(add_parsed_numbers("10", "20"), Ok(30));
        assert!(add_parsed_numbers("abc", "10").is_err());
    }

    #[test]
    fn test_file_reading() {
        fs::write("test.txt", "line 1\nline 2").unwrap();
        // TODO: How can you fix the error in `assert_eq`?
        // "binary operation `==` cannot be applied to type `Result<usize, std::io::Error>`"
        assert_eq!(count_file_lines("test.txt"), Ok(2));
        assert!(count_file_lines("missing.txt").is_err());
        fs::remove_file("test.txt").ok();
    }

    #[test]
    fn test_combined_errors() {
        fs::write("numbers.txt", "5\n10\n15").unwrap();
        assert_eq!(sum_numbers_in_file("numbers.txt"), Ok(30));

        fs::write("bad.txt", "5\nabc\n15").unwrap();
        assert!(sum_numbers_in_file("bad.txt").is_err()); // Parse error
        assert!(sum_numbers_in_file("missing.txt").is_err()); // IO error

        fs::remove_file("numbers.txt").ok();
        fs::remove_file("bad.txt").ok();
    }
}
