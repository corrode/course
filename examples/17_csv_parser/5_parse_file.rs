//! # Parsing a whole file
//!
//! With a working line parser, the file-level parser is mostly
//! plumbing: split on newlines, treat the first line as headers, and
//! parse the rest as data rows.
//!
//! Use [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
//! to split — it handles trailing newlines gracefully, so `"a,b\n"`
//! gives one line, not two.
//!
//! This step composes on top of `parse_csv_line` from the previous
//! step. To keep each step independently runnable, the signature is
//! re-declared here as a stub with `todo!()` — replace it with your
//! solution from step 4 (or just call into it).

/// Parses a CSV line with proper quote handling.
/// (Re-stubbed from step 4 so this file compiles on its own.)
fn parse_csv_line(line: &str) -> Vec<String> {
    todo!()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_file() {
        let content = "name,age,city\nAlice,30,Boston\nBob,25,Seattle";
        let (headers, rows) = parse_csv_file(content);
        assert_eq!(headers, vec!["name", "age", "city"]);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec!["Alice", "30", "Boston"]);
    }
}
