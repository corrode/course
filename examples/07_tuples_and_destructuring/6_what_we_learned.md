# Wrapping up tuples and destructuring

You used tuples to return multiple values, destructured them in
parameter lists and `let` bindings, and saw how ownership behaves
differently for `Copy` and non-`Copy` element types.

## What we learned

- A tuple is a fixed-size group of values whose size and per-slot
  types are part of the type. `(String, u32)` and `(u32, String)`
  are different types.
- Build a tuple with parentheses; access fields with `.0`, `.1`,
  etc. Destructuring with `let (a, b) = pair;` is usually clearer.
- Tuples are the lightest-weight way to return more than one value
  from a function. When the same tuple shows up in many places
  or grows past two or three fields, switch to a `struct`
  (chapter 12).
- Use `_` in a pattern to ignore a field: `let (first, _) = pair;`.
- Move vs. copy still applies: a tuple of `String`s moves on
  destructure, a tuple of integers copies. The element types decide.
- The unit type `()` is the empty tuple. It's what functions
  "without a return value" actually return.
