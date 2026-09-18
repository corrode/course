# Returning multiple values

Functions in Rust return a single value, but a tuple lets you bundle several values into that single return.

Here you'll return a `(String, u32)` pair: a name and an age.

## Useful from the standard library

- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from) or `.to_string()` on a `&str` literal gets you the owned `String` the tuple wants in its first slot.
