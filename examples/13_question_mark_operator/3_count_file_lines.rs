//! # `?` with a different error type
//!
//! Same operator, different error type. File I/O returns
//! [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html);
//! the function signature has to declare it as the error type so `?` is
//! happy passing it through.
//!
//! Notice that `?` doesn't care which error type is involved as long as
//! the function it's used in returns *the same* error type (or one
//! convertible from it via `From`, which is the next step).
//!
//! See: <https://doc.rust-lang.org/std/fs/fn.read_to_string.html>

/// Reads a file and counts lines. Note how `?` works with a different error type.
fn count_file_lines(filename: &str) -> Result<usize, std::io::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
