# Wrapping up the password validator

You've put the whole standard toolkit to work in one project: a struct, an enum, methods, character iteration, vectors of feedback, and a match over score ranges.

## What we learned

- Here, tools you first met separately work together: structs and enums describe the result, iterators inspect the password, and a `Vec<String>` collects feedback.
- Use `s.chars().any(|c| c.is_ascii_*())` for a yes-or-no question and `s.chars().filter(...).count()` when you need the number of matches.
- Membership in a small set of literal characters is one `"!@#$%^&*".contains(c)` call.
  No need for a `HashSet`.
- Range patterns inside `match` arms (`0..30 => Weak`) bucket a number into categories without a chain of `if`/`else` checks.
- Splitting a domain across small types (`PasswordReport`, `PasswordStrength`, `PasswordValidator`) keeps each piece focused on one job and easy to test.
- For real randomness, reach for the [`rand`](https://docs.rs/rand) crate.
  The clock-based trick is fine for an exercise, never for a password generator that ships.
