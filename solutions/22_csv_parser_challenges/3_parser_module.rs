mod csv {
    fn parse_line(line: &str) -> Vec<String> {
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
                    fields.push(std::mem::take(&mut field));
                }
                _ => field.push(c),
            }
        }
        fields.push(field);
        fields
    }

    pub fn parse_file(content: &str) -> (Vec<String>, Vec<Vec<String>>) {
        let mut lines = content.lines();
        let headers = lines.next().map(parse_line).unwrap_or_default();
        let rows = lines.map(parse_line).collect();
        (headers, rows)
    }
}

#[test]
fn public_api_parses_headers_and_rows_with_the_same_rules() {
    let (headers, rows) =
        csv::parse_file("\"last, first\",note\n\"Doe, Jane\",\"said \"\"hi\"\"\"\n");
    assert_eq!(headers, vec!["last, first", "note"]);
    assert_eq!(rows, vec![vec!["Doe, Jane", "said \"hi\""]]);
}

#[test]
fn empty_file_and_headers_only() {
    assert_eq!(csv::parse_file(""), (vec![], vec![]));
    assert_eq!(
        csv::parse_file("a,b\n"),
        (vec!["a".into(), "b".into()], vec![])
    );
}

#[test]
fn multiple_rows_and_crlf() {
    let (headers, rows) = csv::parse_file("a,b\r\n1,2\r\n3,4\r\n");
    assert_eq!(headers, vec!["a", "b"]);
    assert_eq!(rows, vec![vec!["1", "2"], vec!["3", "4"]]);
}
