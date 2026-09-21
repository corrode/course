# Matching One Variant

Sometimes you only care about a single variant. You can still write a full
`match` with a `_` catch-all arm, or you can reach for the `matches!` macro.
Both are idiomatic. For example, `matches!(letter, 'a' | 'e' | 'i' | 'o' | 'u')`
checks a `char` against several patterns and produces a `bool`.

For this exercise, return `true` only for `InternalServerError`, and `false` for
every other variant.

## Useful from the Standard Library

- [`std::matches!`](https://doc.rust-lang.org/std/macro.matches.html) expands to
  a `match` that returns `true` for the given pattern and `false` otherwise.
- [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) enables
  `==` comparisons between values of the enum.
