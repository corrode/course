# Borrow in, own out

Here you borrow text to read it, then return a new `String` that the caller can own (i.e. use however they like).
The signature is very typical: you often get a `&str` in and an owned `String` out.
This is common when a function reads existing string and does some processing to create a new string. 

## Useful from the standard library

- [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase)
  and [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase)
  return new `String`s with the case changed.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from)
  and [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  both create an owned `String` from a `&str`.
  Use whichever reads better.
