/// Reads a file and counts lines. Note how `?` works with a different error type.
fn count_file_lines(filename: &str) -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().count())
}

#[test]
fn test_count_file_lines() {
    use std::fs;
    fs::write("test.txt", "line 1\nline 2").unwrap();
    assert_eq!(count_file_lines("test.txt").unwrap(), 2);
    assert!(count_file_lines("missing.txt").is_err());
    fs::remove_file("test.txt").ok();
}
