# Give the Report a Type

The checks tell us what is present, but the caller will need a summary.
`PasswordReport` groups a numeric score, a list of feedback messages, and a `PasswordStrength` label.
Using an enum for the label means callers can match on known cases instead of comparing strings such as `"strong"` and hoping nobody misspells one.
The report deliberately does not keep a copy of the password.

Implement two methods:

- `PasswordStrength::from_score(score)` returns `Weak` below 30, `Medium` from 30 through 69, and `Strong` from 70 upward.
  It must handle every `u8` value, even though our validator will produce scores no higher than 100.
- `PasswordReport::is_strong(&self)` answers whether the report's stored label is `Strong`.
  It borrows the report, so a caller can ask this question and still read the feedback afterward.

This step doesn't calculate a password's score yet.
The tests supply scores and reports directly so you can check the boundaries independently of the character rules.

## Useful from the Standard Library

- A `match` must cover every variant or value it can receive.
  You can use ranges or conditions to classify the score; choose whichever makes the boundaries clearest to you.
- The supplied `PartialEq` derive allows two `PasswordStrength` values to be compared.
