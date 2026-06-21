/// Takes a mutable reference to modify the string in place.
/// The &mut allows us to change the string's contents.
fn mutate_string(s: &mut String) {
    s.push_str(" - now with extra crab");
}

#[test]
fn test_mutate_string() {
    let mut s = String::from("Ferris");
    mutate_string(&mut s);
    assert_eq!(s, "Ferris - now with extra crab");
}
