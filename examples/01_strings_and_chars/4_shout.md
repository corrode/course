# Borrow in, own out

Here you borrow text to read it, then return a new `String` that the caller can keep.
The signature captures that handoff: `&str` in, owned `String` out.
You'll see the same shape whenever a function reads existing text to produce different text.

## Useful from the standard library

- [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase)
  and [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)
  return new `String`s with the case changed.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from)
  and [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  both create an owned `String` from a `&str`.
  Use whichever reads better.
