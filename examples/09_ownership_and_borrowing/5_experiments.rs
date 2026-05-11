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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experiment_use_after_move() {
        let s = String::from("Rust");
        let _result = take_ownership(s);
        // Uncomment the next line. Expect: "borrow of moved value: `s`".
        // assert_eq!(s, "Rust");
    }

    #[test]
    fn experiment_two_mutable_borrows() {
        let mut s = String::from("Ferris");
        let r1 = &mut s;
        // Uncomment the next two lines together. Expect:
        // "cannot borrow `s` as mutable more than once at a time".
        // let r2 = &mut s;
        // r2.push('!');
        r1.push('!');
    }

    #[test]
    fn experiment_mix_shared_and_mutable() {
        let mut s = String::from("Ferris");
        let shared = &s;
        // Uncomment the next line. Expect:
        // "cannot borrow `s` as mutable because it is also borrowed as immutable".
        // mutate_string(&mut s);
        println!("{shared}");
    }
}
