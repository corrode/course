/// Parses a simple CSV line without quotes by splitting on commas.
fn parse_simple_csv_line(line: &str) -> Vec<String> {
    todo!("Return fields from a comma-separated line without quotes")
}

#[test]
fn test_parse_simple_csv_line() {
    let line = "name,age,city";
    let fields = parse_simple_csv_line(line);
    assert_eq!(fields, vec!["name", "age", "city"]);
}
