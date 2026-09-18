# Wrapping up borrowing

You borrowed a `String` read-only as `&str`, mutated one through `&mut String`, and triggered three ownership and borrowing errors on purpose.

## What we learned

- A borrow lets you read or modify a value without taking ownership.
  `&T` is a shared, read-only borrow; `&mut T` is an exclusive, writable one.
- For the same value, active borrows allow either shared `&T` access or exclusive `&mut T` access, never both.
  This restriction also helps prevent data races across threads.
- Mutability is opt-in at every layer: the binding (`let mut x`), the parameter (`&mut T`), and the call site (`&mut x`).
- Default to `&str` over `&String` (and `&[T]` over `&Vec<T>`) for read-only parameters.
  Slice types accept more callers thanks to deref coercion.
- The compiler errors are part of the lesson.
  Once you can explain one in a sentence, you can decide what ownership or borrow needs to change instead of guessing.
