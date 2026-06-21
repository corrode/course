# Character-class helpers

The validator's scoring rules all boil down to questions like "does this password contain an uppercase letter?"
Before assembling the orchestrator, write the small predicates that answer each one.

All four functions take a `&str` and return `bool`.
The "special" character set for this exercise is `!@#$%^&*`.
Feel free to expand it if you want a stricter validator later.

Hint: `str::chars()` plus `Iterator::any` is the natural way to check for the presence of a character class.

## Useful from the standard library

- [`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars) produces an iterator of `char`s.
  The standard entry point for any per-character check.
- [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) returns `true` as soon as one item satisfies the predicate.
  Reads as `password.chars().any(|c| c.is_ascii_digit())`.
- [`char::is_ascii_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase), [`is_ascii_lowercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_lowercase), and [`is_ascii_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_digit) are the per-character classifiers for three of the four checks.
- For the special-character set, [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains) on the literal `"!@#$%^&*"` (with a `char` argument) gives you a one-line membership test inside the closure.
