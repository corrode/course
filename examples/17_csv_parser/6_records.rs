//! # From rows to records
//!
//! Parallel `Vec`s of headers and row values are awkward to consume.
//! Most code wants to ask "what's the `name` for this row?" — a job for
//! a `HashMap<String, String>` per row.
//!
//! Pair headers with each row using
//! [`Iterator::zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip)
//! and `collect` into a `HashMap`. You'll need `cloned()` on both
//! iterators because the map wants owned `String`s but iteration
//! yields `&String`.

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
