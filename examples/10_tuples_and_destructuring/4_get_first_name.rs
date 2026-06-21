/// Extracts the first and last names from a full name tuple.
/// Takes a tuple (`first_name`, `last_name`) and returns just the first name.
///
/// Note: this signature moves the tuple in (it contains `String`s, which
/// aren't `Copy`). After the call, the original `full_name` is no longer
/// valid; try using it after calling this function and read the error.
/// `swap_values` below takes `(i32, i32)`, which is `Copy`, so the
/// original is still usable. The moves chapter covers this properly.
///
/// Hint: Use [tuple destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
fn get_first_name(full_name: (String, String)) -> String {
    todo!()
}

#[test]
fn test_get_first_name() {
    let full_name = ("John".to_string(), "Doe".to_string());
    assert_eq!(get_first_name(full_name), "John");
}
