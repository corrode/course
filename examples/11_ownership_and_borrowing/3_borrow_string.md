# Borrowing without taking

Most of the time a function only needs to *look at* a value, not own it.
That's a shared borrow, written `&T` in the signature.
The caller keeps ownership; the callee gets temporary read-only access.

This function takes `&str` rather than `&String`.
`&str` is the universal "borrowed string slice" type: a string literal is already a `&'static str`, and `&String` automatically coerces to `&str`, so `&str` parameters accept both without forcing the caller to convert.
Reach for `&str` by default when you're just reading.

The body is a one-liner: call `.len()` on the slice.
The point of the exercise is the signature: notice that after the call, the caller's `s` is still usable in the test below.

## Useful from the standard library

- [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len)
  is the byte length of the slice.
  The chapter on strings covers why that's not the same as a character count.
- The "deref coercion" from `&String` to `&str` is what lets the test pass `&s` directly.
  No `.as_str()` needed.
