//! # Quotes, embedded commas, and escapes
//!
//! Real CSV is a state machine in disguise. A field can be wrapped in
//! double quotes, in which case any commas *inside* the quotes are part
//! of the field, not separators. And a literal `"` inside a quoted
//! field is encoded as `""` (two quotes).
//!
//! Suggested order of attack:
//!
//!   1. Plain `a,b,c` and simply quoted `"a","b","c"` (the basic test).
//!   2. Commas inside quoted fields: `"a,b",c`.
//!   3. Escaped quotes: `"a""b",c` -> [`a"b`, `c`].
//!
//! Walk the string character by character with a peekable iterator and
//! keep a small `in_quotes: bool` flag. When you see `"` while already
//! inside quotes, peek the next char: if it's another `"`, push a
//! literal `"` and consume both; otherwise close the field.

/// Parses a CSV line with proper quote handling.
/// Handles: "field,with,commas", "field with \"quotes\"", etc.
fn parse_csv_line(line: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
