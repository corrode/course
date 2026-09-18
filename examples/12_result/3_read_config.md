# Read Config File

This simulates reading a file: return an error for an empty filename and the fixed content for any other name.
It uses the same pattern as `safe_divide`, but returns an owned `String`.
Notice you can mix `Ok(String::from("..."))` and `Err("...")` in the same function: the success and error types are independent.

## Useful from the Standard Library

- [`str::is_empty`](https://doc.rust-lang.org/std/primitive.str.html#method.is_empty) checks whether the filename is empty.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from) or `.to_string()` turns the literal `"config content"` into the owned `String` the `Ok` arm needs.
