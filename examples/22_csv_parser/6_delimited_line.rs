/// Parse one line using the supplied delimiter and CSV-style quoting.
/// The delimiter is not a quote, CR, or LF; input has balanced quotes.
fn parse_delimited_line(line: &str, delimiter: char) -> Vec<String> {
    todo!()
}

/// Keep the original comma-separated API without duplicating the parsing loop.
fn parse_csv_line(line: &str) -> Vec<String> {
    todo!()
}

#[test]
fn semicolon_fields_and_literal_commas() {
    assert_eq!(parse_delimited_line("a,b;c;", ';'), vec!["a,b", "c", ""]);
}

#[test]
fn delimiter_inside_quotes_is_data() {
    assert_eq!(parse_delimited_line(r#""a;b";c"#, ';'), vec!["a;b", "c"]);
}

#[test]
fn escaped_quote_next_to_delimiter() {
    assert_eq!(
        parse_delimited_line(r#""a;""b";c"#, ';'),
        vec!["a;\"b", "c"]
    );
    assert_eq!(parse_delimited_line("\"\"\"\";\"\"", ';'), vec!["\"", ""]);
}

#[test]
fn tabs_unicode_and_whitespace() {
    assert_eq!(
        parse_delimited_line(" left \tright ", '\t'),
        vec![" left ", "right "]
    );
    assert_eq!(
        parse_delimited_line("é🦀\"a🦀b\"🦀", '🦀'),
        vec!["é", "a🦀b", ""]
    );
}

#[test]
fn empty_fields_are_preserved() {
    assert_eq!(parse_delimited_line("", ';'), vec![""]);
    assert_eq!(parse_delimited_line(";;", ';'), vec!["", "", ""]);
}

#[test]
fn comma_wrapper_keeps_quote_and_escape_behavior() {
    assert_eq!(parse_csv_line(r#""a,b","c""d","#), vec!["a,b", "c\"d", ""]);
}
