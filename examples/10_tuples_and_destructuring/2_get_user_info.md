# Returning multiple values

Functions in Rust return a single value, but a tuple lets you bundle several values into that single return.
It's the lightest-weight way to hand back more than one thing without defining a new type.

Here you'll return a `(String, u32)` pair: a name and an age.

## Useful from the standard library

- [The Rust Book on tuples](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
  covers tuple syntax and how the type signature is just the parenthesized list of element types.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from)
  or `.to_string()` on a `&str` literal gets you the owned `String` the tuple wants in its first slot.
