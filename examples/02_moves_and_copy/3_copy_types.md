# Copy Types Don't Move

`take_ownership` moved a `String`.
With a `Copy` type, you get a copy instead, so the caller keeps its value.

`double` takes an `i32` by value.
Because `i32` is `Copy`, the caller's variable is still alive after the call.
The body is one expression.
Look at the test: it reads `x` again after passing it to `double`.

## Useful from the Standard Library

- `i32`, the other integer types, `bool`, `char`, and fixed-size arrays of `Copy` values all implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html).
  Assigning or passing one duplicates its bits instead of moving it.
