//! # Experiments: get the errors on purpose
//!
//! Passing the previous tests is the easy part of this chapter.
//! Ownership only really clicks once you've seen the canonical errors
//! with your own eyes, so the messages feel familiar later (chapter 11,
//! chapter 13, …) instead of like a brick wall.
//!
//! Each test below is paired with a commented-out line. Uncomment one
//! at a time, run the tests, read the error carefully, then comment it
//! out again before moving on. The errors *are* the lesson here — the
//! tests themselves don't assert anything interesting.
//!
//! The three errors you'll trigger correspond to the three rules of
//! the borrow checker:
//!
//! 1. You can't use a value after you've moved it.
//! 2. You can't have two mutable references to the same value at once.
//! 3. You can't have a mutable reference while a shared reference is
//!    still in use.
//!
//! Re-read each compiler message until you can explain in one sentence
//! *why* the compiler is complaining. That's the muscle this chapter is
//! building.

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
