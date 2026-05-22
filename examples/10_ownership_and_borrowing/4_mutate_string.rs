/// Takes a mutable reference to modify the string in place.
/// The &mut allows us to change the string's contents.
fn mutate_string(s: &mut String) {
    // Append " - now with extra crab" to the string
    todo!()
}

#[test]
fn test_mutate_string() {
    let mut s = String::from("Ferris");
    mutate_string(&mut s);
    assert_eq!(s, "Ferris - now with extra crab");
}
