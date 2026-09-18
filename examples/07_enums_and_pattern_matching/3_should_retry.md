# Matching One Variant

Sometimes you only care about a single variant.
You can still write a full `match` with a `_` catch-all arm, or you can reach for the `matches!` macro.
Both are idiomatic.

For this exercise, return `true` only for `InternalServerError`, and `false` for every other variant.

## Useful from the Standard Library

- [`std::matches!`](https://doc.rust-lang.org/std/macro.matches.html) expands to a `match` that returns `true` for the given pattern and `false` otherwise.
  You can write it as `matches!(status, HttpStatus::InternalServerError)`.
- [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) enables `==`, so the enum's derived implementation also lets you write `status == HttpStatus::InternalServerError`.
