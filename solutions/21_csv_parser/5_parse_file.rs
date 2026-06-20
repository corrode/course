/// Parses a CSV line with proper quote handling.
/// (Re-stubbed from step 4 so this file compiles on its own.)
fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes => {
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

/// Parses a complete CSV file.
/// First line is headers, remaining lines are data.
///
/// Use [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
/// to split on newlines. `lines()` already handles a trailing `\n`
/// gracefully; it won't yield an empty last line for `"a,b\n"`. Real
/// CSVs often end with a newline, so this is the right tool.
/// Returns (headers, rows).
fn parse_csv_file(content: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let mut lines = content.lines();
    let headers = lines.next().map(parse_csv_line).unwrap_or_default();
    let rows = lines.map(parse_csv_line).collect();
    (headers, rows)
}

#[test]
fn test_parse_csv_file() {
    let content = "name,age,city\nAlice,30,Boston\nBob,25,Seattle";
    let (headers, rows) = parse_csv_file(content);
    assert_eq!(headers, vec!["name", "age", "city"]);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["Alice", "30", "Boston"]);
}
