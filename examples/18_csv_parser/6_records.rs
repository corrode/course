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
