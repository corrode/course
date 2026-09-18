# Borrow In, Own Out

Here you borrow text to read it, then return an uppercase `String` that the caller owns.
You'll often see `&str` in and `String` out when a function reads existing text to produce new text.

## Useful from the Standard Library

- [`str::to_uppercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase) returns a new `String` with the text converted to uppercase.
