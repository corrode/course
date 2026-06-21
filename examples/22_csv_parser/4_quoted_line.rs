/// Parses a CSV line with proper quote handling.
/// Handles: "field,with,commas", "field with \"quotes\"", etc.
fn parse_csv_line(line: &str) -> Vec<String> {
    todo!()
}

#[test]
fn test_parse_csv_line_plain_numbers() {
    let line = r#"1,2,3"#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec!["1", "2", "3"]);
}

#[test]
fn test_parse_csv_line_plain_strings() {
    // Warm-up: every field is quoted, no commas inside, no escapes.
    // Get this passing first; it forces you to enter and exit a quoted
    // field, but nothing trickier.
    let line = r#""a","b","c""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_parse_csv_line_quoted() {
    let line = r#"name,"age, years","city""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec!["name", "age, years", "city"]);
}

#[test]
fn test_parse_csv_line_quoted_escaped() {
    let line = r#""John ""Johnny"" Doe","25","New York""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec![r#"John "Johnny" Doe"#, "25", "New York"]);
}
