/// Reads a configuration file (simulated).
/// Returns Ok(content) normally, Err("File not found") for empty input.
///
/// Same idea as `safe_divide`, but returning an owned `String`. Notice
/// you can mix `Ok(String::from("..."))` and `Err("...")` in the same
/// function: the success and error types are independent.
fn read_config_file(filename: &str) -> Result<String, &'static str> {
    if filename.is_empty() {
        Err("File not found")
    } else {
        Ok(String::from("config content"))
    }
}

#[test]
fn test_read_config_file() {
    assert_eq!(
        read_config_file("app.toml"),
        Ok("config content".to_string())
    );
    assert!(read_config_file("").is_err());
}
