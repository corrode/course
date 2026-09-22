/// Parses a simple CSV line without quotes by splitting on commas.
fn parse_simple_csv_line(line: &str) -> Vec<String> {
    line.split(',').map(str::to_string).collect()
}

#[test]
fn test_parse_simple_csv_line() {
    let line = "name,age,city";
    let fields = parse_simple_csv_line(line);
    assert_eq!(fields, vec!["name", "age", "city"]);
}
