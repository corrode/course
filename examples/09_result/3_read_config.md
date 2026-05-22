# Read config file

Same shape as `safe_divide`, but returning an owned `String`. Notice
you can mix `Ok(String::from("..."))` and `Err("...")` in the same
function: the success and error types are independent.

## Useful from the standard library

- [`str::is_empty`](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty)
  is the cleanest way to detect an empty filename.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from)
  or `.to_string()` turns the literal `"config content"` into the
  owned `String` the `Ok` arm needs.
