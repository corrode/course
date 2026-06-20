# The orchestrator: `validate`

Time to combine everything. `PasswordValidator::validate(password)`
returns a `PasswordReport` with a numeric score, a list of feedback
messages, and a `PasswordStrength` label.

Because each step in this chapter stands on its own, you'll re-implement
the helpers from earlier steps here so this step can stand on its own.
The shared types and the four `has_*` helpers are stubbed below. Fill
them in (or copy your earlier solutions) and then write `validate` on
top of them.

Suggested scoring (feel free to tweak; the tests only check broad
ranges):

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

Push a short message into `feedback` for every rule that *fails*. That
way `PasswordAdvisor` (or your own future code) has something
to react to. The length-related complaint should mention "characters",
"length", "short", "longer", or "at least" so the test below can
recognise it.

## Useful from the standard library

- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
  is byte length, but for an ASCII-only check it's also the
  character count. Good enough for the length thresholds.
- [`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
  for the `feedback` accumulator; push a `String` for every failed
  rule.
- A `match` on the final score with range patterns
  (`0..30 => Weak, 30..70 => Medium, _ => Strong`) keeps the
  classification clean. Range patterns are end-exclusive by default;
  use `..=` if you want the upper bound included.
- The character-class helpers are exactly what you wrote in step 4,
  so the body of `validate` is mostly bookkeeping: add to `score`,
  push to `feedback`, then build the report.
