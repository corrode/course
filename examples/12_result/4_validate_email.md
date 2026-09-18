# Validate email

Now the `Ok` value is a borrow of the input.
The `&str` in the return type implicitly borrows from `email`, so the compiler infers a lifetime linking input and output via lifetime elision.
The memory and ownership chapter explains why a returned reference must not outlive the value it borrows.

## Useful from the standard library

- [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains) takes a `char` (or another `&str`) and answers yes/no.
  So `email.contains('@')` is exactly the check you need.
- The `Ok` branch can return the input slice directly: it's already a `&str` with the right lifetime.
  You don't need to allocate a `String` with `to_string()`.
