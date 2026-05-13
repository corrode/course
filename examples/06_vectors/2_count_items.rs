/// Gets the number of items in the list.
///
/// Start here. The simplest possible operation: ask a `Vec` how long it
/// is. Notice the parameter is `&Vec<String>`: a borrow, so the caller
/// keeps ownership.
/// See: <https://doc.rust-lang.org/std/vec/struct.Vec.html>
fn count_items(list: &Vec<String>) -> usize {
    todo!()
}

#[test]
fn test_count_items() {
    let list = vec!["apple".to_string(), "banana".to_string()];
    assert_eq!(count_items(&list), 2);
}
