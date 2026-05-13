# Mapping variants to values

Your first `match`: turn each `HttpStatus` variant into the numeric
code it represents. The compiler will complain if you forget a
variant, which is exactly what you want.

## Useful from the standard library

- [The Rust Book on `match`](https://doc.rust-lang.org/book/ch06-02-match.html)
  is the reference for pattern syntax: literal patterns, `|` for
  multiple patterns, ranges, and the `_` catch-all.
- [`std::cmp::PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
  is the trait that lets you use `==` on a value. `#[derive(PartialEq)]`
  on the enum asks the compiler to write the implementation for you,
  which is what makes `assert_eq!` in the tests work.
- [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
  enables `{:?}` formatting. Handy when a test fails and you want to
  see which variant came back.
