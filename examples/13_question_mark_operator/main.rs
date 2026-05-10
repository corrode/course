//! # The Question Mark Operator
//!
//! The question mark operator is Rust's shorthand for "if this is an error,
//! return it from the current function; otherwise, unwrap the value and
//! continue." It replaces a lot of `match` boilerplate.
//!
//! This exercise builds on `Result<T, E>` from the previous lesson.

/// Adds two parsed numbers. Compare this to doing it with match statements.
/// See: https://doc.rust-lang.org/std/primitive.str.html#method.parse
fn add_parsed_numbers(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

#[test]
fn test_add_parsed_numbers() {
    assert_eq!(add_parsed_numbers("10", "20"), Ok(30));
    assert!(add_parsed_numbers("abc", "10").is_err());
}

/// Reads a file and counts lines. Note how `?` works with a different error type.
/// See: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
fn count_file_lines(filename: &str) -> Result<usize, std::io::Error> {
    todo!()
}

#[test]
fn test_count_file_lines() {
    use std::fs;
    fs::write("test.txt", "line 1\nline 2").unwrap();
    // Once `count_file_lines` returns `Ok(2)`, this assertion succeeds:
    // a `Result<T, E>` compares equal to `Ok(value)` via its derived
    // `PartialEq`. No change needed below; fix `count_file_lines` above.
    assert_eq!(count_file_lines("test.txt"), Ok(2));
    assert!(count_file_lines("missing.txt").is_err());
    fs::remove_file("test.txt").ok();
}

/// Combines parsing and file I/O errors using `Box<dyn Error>`.
/// This shows how `?` can handle multiple error types in one function.
/// See: https://doc.rust-lang.org/std/error/trait.Error.html
fn sum_numbers_in_file(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    todo!()
}

#[test]
fn test_sum_numbers_in_file() {
    use std::fs;
    fs::write("numbers.txt", "5\n10\n15").unwrap();
    assert_eq!(sum_numbers_in_file("numbers.txt"), Ok(30));

    fs::write("bad.txt", "5\nabc\n15").unwrap();
    assert!(sum_numbers_in_file("bad.txt").is_err()); // Parse error
    assert!(sum_numbers_in_file("missing.txt").is_err()); // IO error

    fs::remove_file("numbers.txt").ok();
    fs::remove_file("bad.txt").ok();
}
