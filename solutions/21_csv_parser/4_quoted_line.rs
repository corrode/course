/// Parses a CSV line with proper quote handling.
/// Handles: "field,with,commas", "field with \"quotes\"", etc.
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes => {
                // A doubled quote inside a quoted field is a literal quote;
                // a lone quote ends the quoted section.
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            }
            '"' => in_quotes = true,
            ',' if !in_quotes => {
                fields.push(field.clone());
                field.clear();
            }
            _ => field.push(c),
        }
    }
    fields.push(field);
    fields
}

#[test]
fn test_parse_csv_line_basic() {
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
fn test_parse_csv_line_escaped_quotes() {
    let line = r#""John ""Johnny"" Doe","25","New York""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec![r#"John "Johnny" Doe"#, "25", "New York"]);
}
