# The orchestrator: `validate`

Time to combine everything. `PasswordValidator::validate(password)`
returns a `PasswordReport` with a numeric score, a list of feedback
messages, and a `PasswordStrength` label.

Because each step in this chapter stands on its own, you'll re-implement
the helpers from earlier steps here so this step can stand on its own.
The shared types and the four `has_*` helpers are stubbed below — fill
them in (or copy your earlier solutions) and then write `validate` on
top of them.

Suggested scoring (feel free to tweak — the tests only check broad
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

Push a short message into `feedback` for every rule that *fails* — that
way [`PasswordAdvisor`](super) (or your own future code) has something
to react to. The length-related complaint should mention "characters",
"length", "short", "longer", or "at least" so the test below can
recognise it.
