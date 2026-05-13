/// Ferris the crab has moods. Decide which one based on how
/// hungry he is (a `0..=10` scale) and how many naps he's had today.
///
/// The rules, in plain English:
///
/// - If Ferris is **very** hungry (8 or more), he's `"Hangry"`,
///   no matter how many naps he's had.
/// - Otherwise, if he's also a bit hungry (5 or more) **and** has
///   had no naps, he's `"Grumpy"`.
/// - Otherwise, if he's had three or more naps, he's `"Sleepy"`.
/// - Otherwise, he's `"Content"`.
///
/// One `if`/`else if`/`else` chain, returning a `&'static str`.
/// The order of the branches matters; the tests will catch you if
/// you get it wrong.
fn ferris_mood(hunger: u32, naps: u32) -> &'static str {
    todo!()
}

#[test]
fn content_by_default() {
    assert_eq!(ferris_mood(3, 1), "Content");
    assert_eq!(ferris_mood(0, 2), "Content");
}

#[test]
fn grumpy_when_hungry_and_napless() {
    assert_eq!(ferris_mood(5, 0), "Grumpy");
    assert_eq!(ferris_mood(7, 0), "Grumpy");
}

#[test]
fn sleepy_after_too_many_naps() {
    assert_eq!(ferris_mood(2, 3), "Sleepy");
    assert_eq!(ferris_mood(0, 10), "Sleepy");
}

#[test]
fn hangry_overrides_everything() {
    // Ferris is too hungry to care about anything else.
    // If you check naps before hunger, these will fail.
    assert_eq!(ferris_mood(8, 0), "Hangry");
    assert_eq!(ferris_mood(9, 5), "Hangry");
    assert_eq!(ferris_mood(10, 100), "Hangry");
}
