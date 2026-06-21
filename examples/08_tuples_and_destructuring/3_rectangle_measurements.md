# Computing two values at once

When two results are naturally produced together, returning them as a tuple is often clearer than two separate function calls.
The caller destructures the result into named bindings.

## Useful from the standard library

- The arithmetic operators `*` and `+` are all you need here.
  Both `u32` results fit easily for any sane rectangle.
- Tuple construction is just parentheses: `(area, perimeter)`.
  The return type `(u32, u32)` already tells the compiler what shape to expect.
- The caller in the test uses `let (area, perimeter) = ...` to destructure the return into named bindings, the mirror image of how you build it.
