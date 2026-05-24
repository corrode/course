# Wrapping up the password validator

You've put the whole standard toolkit to work in one project: a
struct, an enum, methods, character iteration, vectors of feedback,
a match over score ranges, and a tiny reflection of how the pieces
talk to each other.

## What we learned

- Open-ended projects are where the chapters since chapter 1 start
  to feel cohesive. The same handful of types (struct, enum, `Vec`,
  `String`, `Option`) keep showing up.
- Per-character checks are almost always
  `s.chars().any(|c| c.is_ascii_*())` or
  `s.chars().filter(...).count()`. Internalise this call chain.
- Membership in a small set of literal characters is one
  `"!@#$%^&*".contains(c)` call. No need for a `HashSet`.
- A `Vec<String>` you `push` into as you check each rule is the
  idiomatic way to accumulate validation feedback.
- Range patterns inside `match` arms (`0..30 => Weak`) are the
  cleanest way to bucket a number into categories.
- Splitting a domain across small types (`PasswordReport`,
  `PasswordStrength`, `PasswordValidator`, `PasswordAdvisor`) keeps
  each piece focused on one job and easy to test.
- For real randomness, reach for the [`rand`](https://docs.rs/rand)
  crate. The clock-based trick is fine for an exercise, never for a
  password generator that ships.
