# Wrapping Up Borrowing

You borrowed a `String` read-only as `&str`, mutated one through `&mut String`, and practiced repairing ownership and borrowing errors.

## What We Learned

- A borrow lets you read or modify a value without taking ownership.
  `&T` is a shared, read-only borrow; `&mut T` is an exclusive, writable one.
- For the same value, active borrows allow either shared `&T` access or exclusive `&mut T` access, never both.
  This restriction also helps prevent data races across threads.
- For these local references, a borrow ends at the reference's last use, not necessarily at the closing brace.
  Using `r1` before creating `r2` lets their exclusive borrows take turns.
  Printing `shared` before mutating `s` ends the shared borrow before the mutable one starts.
  Using either earlier reference again after the mutation can bring the conflict back.
- Mutability is opt-in at every layer: the binding (`let mut x`), the parameter (`&mut T`), and the call site (`&mut x`).
- Default to `&str` over `&String` (and `&[T]` over `&Vec<T>`) for read-only parameters.
  Slice types accept more callers thanks to deref coercion.
- The compiler errors are part of the lesson.
  Once you can explain one in a sentence, you can decide what ownership or borrow needs to change instead of guessing.
