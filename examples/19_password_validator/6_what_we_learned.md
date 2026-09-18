# Wrapping Up the Password Validator

You built a report with a struct and an enum, checked characters with iterators, and collected feedback in a vector.
Methods and a `match` over score ranges tied those pieces together.

## What We Learned

- Use `s.chars().any(|c| c.is_ascii_*())` for a yes-or-no question and `s.chars().filter(...).count()` when you need the number of matches.
- Membership in a small set of literal characters is one `"!@#$%^&*".contains(c)` call.
  You don't need a `HashSet` for that.
- Range patterns inside `match` arms (`0..30 => Weak`) bucket a number into categories without a chain of `if`/`else` checks.
- Splitting a domain across small types (`PasswordReport`, `PasswordStrength`, `PasswordValidator`) keeps each piece focused on one job and easy to test.
- For real randomness, reach for the [`rand`](https://docs.rs/rand) crate.
  The clock-based trick is fine for an exercise, never for a password generator that ships.
