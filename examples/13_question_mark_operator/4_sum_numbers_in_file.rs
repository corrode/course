//! # `?` across multiple error types
//!
//! Now the function combines two distinct error sources: parsing a
//! string into a number ([`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html))
//! and reading a file ([`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)).
//! With a fixed error type you'd need to convert each one by hand.
//! `Box<dyn std::error::Error>` is the laziest catch-all: any type that
//! implements the [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
//! trait can be boxed into it, and `?` does the conversion automatically
//! because both `ParseIntError` and `io::Error` implement `Error`.
//!
//! In real code you'd typically define your own error enum (or reach for
//! a crate like `anyhow` or `thiserror`). `Box<dyn Error>` is the
//! standard-library escape hatch.
//!
//! See: <https://doc.rust-lang.org/std/error/trait.Error.html>

/// Combines parsing and file I/O errors using `Box<dyn Error>`.
/// This shows how `?` can handle multiple error types in one function.
fn sum_numbers_in_file(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_numbers_in_file() {
        use std::fs;
        fs::write("numbers.txt", "5\n10\n15").unwrap();
        assert_eq!(sum_numbers_in_file("numbers.txt").unwrap(), 30);

        fs::write("bad.txt", "5\nabc\n15").unwrap();
        assert!(sum_numbers_in_file("bad.txt").is_err()); // Parse error
        assert!(sum_numbers_in_file("missing.txt").is_err()); // IO error

        fs::remove_file("numbers.txt").ok();
        fs::remove_file("bad.txt").ok();
    }
}
