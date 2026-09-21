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
    let _result = take_ownership(s);
    // Will this assertion compile? Make a prediction, then uncomment it and
    // test.
    // assert_eq!(s, "Rust");
}

#[test]
fn experiment_two_mutable_borrows() {
    let mut s = String::from("Ferris");
    let r1 = &mut s;
    // Will these borrows compile? Predict, uncomment both lines, then test.
    // Fix the error by reordering the statements.
    // let r2 = &mut s;
    // r2.push('!');
    r1.push('!');
}

#[test]
fn experiment_mix_shared_and_mutable() {
    let mut s = String::from("Ferris");
    let shared = &s;
    // Will this mutation compile? Make a prediction, then uncomment it and
    // test.
    // Fix the error by reordering the statements.
    // mutate_string(&mut s);
    println!("{shared}");
}
