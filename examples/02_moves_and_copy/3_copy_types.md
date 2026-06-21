# Copy types don't move

`take_ownership` moved a `String`.
This step shows the other half of the rule: a `Copy` type is duplicated instead, so the caller keeps its value.

`double` takes an `i32` by value.
Because `i32` is `Copy`, the caller's variable is still alive after the call.
The body is one expression; the lesson is in the test, where `x` is read again after being passed in.

## Useful from the standard library

- `i32`, the other integer types, `bool`, `char`, and fixed-size arrays of `Copy` values all implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html).
  Assigning or passing one duplicates its bits instead of moving it.
- Heap-owning types like `String` and `Vec<T>` are deliberately not `Copy`.
  When you need a second owner of one of those, ask for it with [`Clone::clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone).
