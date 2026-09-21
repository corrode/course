fn parse_delimited_line(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut field = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if in_quotes => {
                // A doubled quote inside a quoted field is a literal quote; a
                // lone quote ends the quoted section.
                if chars.peek() == Some(&'"') {
                    field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            }
            '"' => in_quotes = true,
            c if c == delimiter && !in_quotes => {
                fields.push(std::mem::take(&mut field));
            }
            _ => field.push(c),
        }
    }
    fields.push(field);
    fields
}

fn parse_csv_line(line: &str) -> Vec<String> {
    parse_delimited_line(line, ',')
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
