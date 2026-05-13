# Greeting by name

Your first function. Take a `&str` (a borrowed string slice) and
return a `String` formatted as `"Hello, <name>!"`. The signature is
the warmup; the body is one `format!` call.

## Useful from the standard library

- [`format!`](https://doc.rust-lang.org/std/macro.format.html) builds
  a new `String` from a template and arguments. Same syntax as
  `println!`, but returns the string instead of printing it.
- The body is one expression. Leave off the trailing semicolon and
  the `format!` value becomes the return value.
