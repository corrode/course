# Validation Rules

Two structs can share a validation contract without sharing a base class.
The `Validator` trait requires a `check` method that borrows both the rule and the input.
It returns `Ok(())` when the input passes or `Err(String)` with a failure message.

Write a `Validator` implementation for each supplied struct at its comment marker.
There is no worked implementation to copy in this step.

- `MustContain` succeeds when the input contains `needle` and otherwise returns `must contain '<needle>'`.
- `MustNotContain` succeeds when the input does not contain `forbidden` and otherwise returns `must not contain '<forbidden>'`.

Replace the angle-bracket placeholders with the configured text, keeping the single quotes.
For example, a required `@` that is missing produces `must contain '@'`.
Match substrings, not just whole inputs, and keep the comparison case-sensitive.
Use the usual string containment behavior for empty patterns too: every string, including the empty string, contains the empty pattern.

The tests will report missing trait implementations until you add them.
That is expected; the starter has no incomplete Rust items outside the tests.
Keep the trait signature unchanged so either rule can be used wherever a `Validator` is required.
