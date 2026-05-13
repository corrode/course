/// Reads a file and counts lines. Note how `?` works with a different error type.
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
