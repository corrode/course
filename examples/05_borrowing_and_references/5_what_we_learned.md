# Wrapping up borrowing

You borrowed a `String` read-only as `&str`, mutated one through `&mut String`, and triggered the borrow checker's three canonical errors on purpose.

## What we learned

- A borrow lets you read or modify a value without taking ownership.
  `&T` is a shared, read-only borrow; `&mut T` is an exclusive, writable one.
- The borrow-checker rule: at any moment, either any number of `&T` borrows or exactly one `&mut T`, never both.
  That's what rules out data races at compile time.
- Mutability is opt-in at every layer: the binding (`let mut x`), the parameter (`&mut T`), and the call site (`&mut x`).
- Default to `&str` over `&String` (and `&[T]` over `&Vec<T>`) for read-only parameters.
  Slice types accept more callers thanks to deref coercion.
- The compiler errors are the lesson.
  Once you can say in one sentence why the compiler is complaining, you've built the muscle this chapter is for.
