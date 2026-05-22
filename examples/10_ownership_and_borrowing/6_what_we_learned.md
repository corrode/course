# Wrapping up ownership and borrowing

You moved a `String` into a function and back out, borrowed one
read-only as `&str`, mutated one through `&mut String`, and made
the borrow checker complain on purpose to see its three canonical
errors.

## What we learned

- Every value has exactly one owner. When the owner goes out of
  scope, the value is dropped. No garbage collector required.
- Assigning or passing a non-`Copy` value transfers ownership. The
  old binding is no longer usable. `Copy` types (integers, bools,
  chars, fixed-size arrays of those) are duplicated bit-for-bit
  instead.
- Borrows let you read or modify a value without taking ownership.
  `&T` is a shared (read-only) borrow; `&mut T` is exclusive.
- The borrow-checker rule: at any time, either any number of `&T`
  borrows or exactly one `&mut T` borrow, never both. That's what
  rules out data races at compile time.
- Mutability is opt-in at every layer: the binding (`let mut x`),
  the parameter (`mut s: String` or `&mut T`), and the call site
  (`&mut x`).
- Default to `&str` over `&String` (and `&[T]` over `&Vec<T>`) for
  read-only parameters. Slice types accept more callers thanks to
  deref coercion.
- `clone()` is the explicit escape hatch when you really do need two
  owners. Reach for it after you understand why a borrow won't
  compile, not before.
