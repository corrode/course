/// Adds two parsed numbers. Compare this to doing it with match statements.
fn add_parsed_numbers(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;
    Ok(a + b)
}

#[test]
fn test_add_parsed_numbers() {
    assert_eq!(add_parsed_numbers("10", "20"), Ok(30));
    assert!(add_parsed_numbers("abc", "10").is_err());
}
