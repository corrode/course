/// Combines parsing and file I/O errors using `Box<dyn Error>`.
/// This shows how `?` can handle multiple error types in one function.
fn sum_numbers_in_file(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let mut sum = 0;
    for line in content.lines() {
        sum += line.trim().parse::<i32>()?;
    }
    Ok(sum)
}

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
