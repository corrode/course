# Iterating over characters

Strings aren't directly indexable in Rust, because UTF-8 characters have varying widths, but you can walk through their `char`s.
A `for c in text.chars()` loop works.
The exercise asks whether the text contains an ASCII uppercase letter (`A` through `Z`).
You can use `any()`, which stops as soon as it finds a match.

## Useful from the standard library

- [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) returns `true` if any item in the iterator matches a predicate.
  It stops at the first match, so you don't have to inspect the rest of the string.
- [`char::is_ascii_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase) checks whether a character is `A` through `Z`.
  It returns `false` for non-ASCII letters such as `É`, as this exercise requires.
