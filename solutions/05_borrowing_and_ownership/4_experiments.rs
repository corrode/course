/// Takes ownership of a String and modifies it.
fn take_ownership(s: String) -> String {
    let mut s = s;
    s.push_str(" - owned by Rust!");
    s
}

/// Takes a mutable reference to modify the string in place.
fn mutate_string(s: &mut String) {
    s.push_str(" - now with extra crab");
}

#[test]
fn experiment_use_after_move() {
    let s = String::from("Rust");
    let result = take_ownership(s);
    // `s` moved into the function; `result` now owns the returned string.
    assert_eq!(result, "Rust - owned by Rust!");
}

#[test]
fn experiment_two_mutable_borrows() {
    let mut s = String::from("Ferris");
    let r1 = &mut s;
    r1.push('!');
    // The borrow through `r1` ends at its last use, before we create `r2`.
    let r2 = &mut s;
    r2.push('!');
    assert_eq!(s, "Ferris!!");
}

#[test]
fn experiment_mix_shared_and_mutable() {
    let mut s = String::from("Ferris");
    let shared = &s;
    println!("{shared}");
    // Nothing uses `shared` after this point, so we can borrow `s` mutably.
    mutate_string(&mut s);
    assert_eq!(s, "Ferris - now with extra crab");
}
