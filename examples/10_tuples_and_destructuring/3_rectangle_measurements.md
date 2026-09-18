# Computing two values at once

When two results are naturally produced together, returning them as a tuple is often clearer than two separate function calls.
The caller destructures the result into named bindings.

## Useful from the standard library

- The arithmetic operators `*` and `+` are all you need here.
  The dimensions in the tests keep both `u32` results within range.
- Tuple construction is just parentheses: `(area, perimeter)`.
  The return type `(u32, u32)` tells the compiler to expect two `u32` values.
- The caller in the test uses `let (area, perimeter) = ...` to pull the two values back out into named bindings.
