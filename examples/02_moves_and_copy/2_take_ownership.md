# Taking ownership by value

When a function parameter has an owned type like `String` (no `&` in front), calling the function *moves* the argument in.
The caller's binding is no longer usable afterwards.
The value now belongs to the called function.
It will be dropped when that function finishes unless the function hands ownership back, which is exactly what the return value does here.

Read the signature as `String` in, `String` out.
The function can choose to mutate the value it owns, but the parameter binding still needs `mut` before you can change it.
Append the text to `s`, then return the same `String` so the caller owns it again.

## Useful from the standard library

- [`String::push_str`](https://doc.rust-lang.org/std/string/struct.String.html#method.push_str)
  appends a `&str` to an owned `String`.
  No allocation if there's spare capacity.
- The parameter needs `mut s: String` to call `push_str` on it.
  Mutability is a property of the binding, not the type, so even an owned value has to be declared `mut` before you can mutate it.
  The `mut` is local to the function and doesn't appear in the type.
