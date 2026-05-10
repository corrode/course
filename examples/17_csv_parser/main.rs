//! # CSV Parser
//!
//! CSV (Comma-Separated Values) has been around since the early days of
//! computing. It's one of those formats that looks trivial but has surprising
//! complexity once you start handling quotes, escapes, and embedded commas.
//!
//! Edgar Codd, who invented the relational database model, would probably
//! be amazed at how his ideas about tabular data ended up in this universal
//! data exchange format. From Excel exports to data science pipelines, CSV
//! is everywhere.
//!
//! This exercise pulls together strings, parsing, error handling, collections,
//! and iteration. Good luck.

/// Parses a simple CSV line without quotes.
/// Splits on commas and trims whitespace.
fn parse_simple_csv_line(line: &str) -> Vec<String> {
    todo!()
}

#[test]
fn test_simple_csv() {
    let line = "name, age, city";
    let fields = parse_simple_csv_line(line);
    assert_eq!(fields, vec!["name", "age", "city"]);
}

/// Parses a CSV line with proper quote handling.
/// Handles: "field,with,commas", "field with \"quotes\"", etc.
fn parse_csv_line(line: &str) -> Vec<String> {
    // This is challenging! Suggested order of attack:
    //
    //   1. Handle the easy cases first: plain `a,b,c` and simply quoted
    //      `"a","b","c"`. The first test below covers exactly this.
    //   2. Handle commas inside quoted fields: `"a,b",c`.
    //   3. Finally, handle escaped quotes inside a quoted field, where
    //      `""` means a literal `"`: `"a""b",c` -> [`a"b`, `c`].
    //
    // The state-machine skeleton in the chapter intro is your friend.
    // Walk character by character, keep a small `in_quotes: bool`, and
    // peek the next char (`chars.peekable()`) to spot the `""` escape.
    todo!()
}

#[test]
fn test_quoted_csv_basic() {
    // Warm-up: every field is quoted, no commas inside, no escapes.
    // Get this passing first; it forces you to enter and exit a quoted
    // field, but nothing trickier.
    let line = r#""a","b","c""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec!["a", "b", "c"]);
}

#[test]
fn test_quoted_csv() {
    let line = r#"name,"age, years","city""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec!["name", "age, years", "city"]);
}

#[test]
fn test_csv_with_quotes_inside() {
    let line = r#""John ""Johnny"" Doe","25","New York""#;
    let fields = parse_csv_line(line);
    assert_eq!(fields, vec![r#"John "Johnny" Doe"#, "25", "New York"]);
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
    todo!()
}

#[test]
fn test_complete_csv() {
    let content = "name,age,city\nAlice,30,Boston\nBob,25,Seattle";
    let (headers, rows) = parse_csv_file(content);
    assert_eq!(headers, vec!["name", "age", "city"]);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0], vec!["Alice", "30", "Boston"]);
}

/// Converts CSV data to a vector of records (HashMap).
/// Each record maps column names to values.
///
/// Hint: pair headers with each row using
/// [`Iterator::zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip),
/// then `collect` into a `HashMap<String, String>`. Sketch:
///
/// ```ignore
/// rows.iter()
///     .map(|row| {
///         headers.iter().cloned()
///             .zip(row.iter().cloned())
///             .collect::<HashMap<_, _>>()
///     })
///     .collect()
/// ```
///
/// `cloned()` is needed because the `HashMap` wants owned `String`s but
/// the iterators are yielding `&String`.
fn csv_to_records(
    headers: &[String],
    rows: &[Vec<String>],
) -> Vec<std::collections::HashMap<String, String>> {
    todo!()
}

#[test]
fn test_csv_to_records() {
    let headers = vec!["name".to_string(), "age".to_string()];
    let rows = vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ];
    let records = csv_to_records(&headers, &rows);
    assert_eq!(records[0].get("name"), Some(&"Alice".to_string()));
    assert_eq!(records[0].get("age"), Some(&"30".to_string()));
}
