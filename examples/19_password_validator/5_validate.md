# Putting the Checks Together

`PasswordValidator::validate(password)` combines the character checks and scoring rules into a `PasswordReport` with a numeric score, a list of feedback messages, and a `PasswordStrength` label.

The shared types are declared below, and the four `has_*` character-class helpers are stubbed.
Fill in the helpers (the intro shows the `.chars().any(...)` pattern), copy your `is_strong` implementation, and then write `validate`.

Here's a suggested scoring scheme.
You can adjust it as long as the results stay within the broad ranges the tests check:

- At least 8 characters: +20
- Contains uppercase: +15
- Contains lowercase: +15
- Contains a digit: +15
- Contains a special char from `!@#$%^&*`: +15
- At least 12 characters: +10
- At least 16 characters: +10

Map the final score to `PasswordStrength`:
- `< 30` → `Weak`
- `30..70` → `Medium`
- `>= 70` → `Strong`

Push a short message into `feedback` for every rule that *fails*.
That way your own follow-up code has something to react to.
The length-related complaint should mention "characters", "length", "short", "longer", or "at least" so the test below can recognise it.

## Useful from the Standard Library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) followed by `.count()` gives the length in Unicode scalar values.
  `str::len` counts bytes, which can overestimate the length of non-ASCII passwords.
- [`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new) for the `feedback` accumulator; push a `String` for every failed rule.
- A `match` on the final score with range patterns (`0..30 => Weak, 30..70 => Medium, _ => Strong`) keeps the classification clean.
  Range patterns are end-exclusive by default; use `..=` if you want the upper bound included.
- The four character-class helpers are the `.chars().any(...)` predicates from the intro, so the body of `validate` is mostly bookkeeping: add to `score`, push to `feedback`, then build the report.
