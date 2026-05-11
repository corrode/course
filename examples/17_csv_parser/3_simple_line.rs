//! # A first pass: comma-splitting
//!
//! Before tackling the messy realities of CSV (quotes, escapes, embedded
//! commas), let's handle the trivial case: a line that's nothing but
//! plain values separated by commas, possibly with surrounding
//! whitespace. This is what most "I'll just split on commas" CSV
//! parsers do — and it's also why so many of them break.
//!
//! Use [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split)
//! and [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).
//! Collect into a `Vec<String>`.

/// Parses a simple CSV line without quotes.
/// Splits on commas and trims whitespace.
fn parse_simple_csv_line(line: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_csv_line() {
        let line = "name, age, city";
        let fields = parse_simple_csv_line(line);
        assert_eq!(fields, vec!["name", "age", "city"]);
    }
}
