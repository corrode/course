# Iterating over characters

Strings aren't directly indexable in Rust (because UTF-8 characters have varying widths), but you can iterate over their `char`s.
A plain `for c in text.chars()` loop will work, and so will the iterator combinators like `any` or `find`, which usually express "is there at least one ..." checks more directly.

## Useful from the standard library

- [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) returns `true` if any item in the iterator matches a predicate.
  Stops at the first match, so it's cheap.
- [`char::is_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_uppercase)
  and
  [`char::is_ascii_uppercase`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase)
  classify a single character.
  The Unicode-aware version is the safer default; the ASCII version is faster when you know the input is ASCII.
