# Borrow in, own out

This step is the canonical "borrowed in, owned out" pattern.
The caller hands you a cheap `&str` view, and you give back a brand new `String` that they get to keep.
You'll see this pattern over and over in real Rust code, so it's worth getting comfortable with the signature now.

## Useful from the standard library

- [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase)
  and [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)
  return new `String`s with the case changed.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from)
  and [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  both create an owned `String` from a `&str`.
  Use whichever reads better.
