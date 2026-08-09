# Borrowing without taking

Most of the time a function only needs to *look at* a value, not own it.
That's a shared borrow, written `&T` in the signature.
The caller keeps ownership; the callee gets temporary read-only access.

This function takes `&str` rather than `&String`.
A string literal is already a `&str`, and a borrowed `String` automatically becomes one at the call site.
That means the same parameter accepts both without making the caller convert anything.
Reach for `&str` by default when you only need to read text.

The body only needs to call `.len()` on the slice.
Pay more attention to the signature and the test: after the call, the caller can still use `s` because the function only borrowed it.

## Useful from the standard library

- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
  is the byte length of the slice.
  The chapter on strings covers why that's not the same as a character count.
- Rust calls the automatic conversion from `&String` to `&str` a "deref coercion."
  It is why the test can pass `&s` directly, with no `.as_str()` call.
