# Wrapping up functions

Across these exercises, you used three parts of Rust's function model:

- **`stray_semicolon`** drove home the difference between an expression and a statement: one trailing `;` is the line between "return this value" and "return `()`".
- **`sum_to`** built a value with recursion, where each call's answer feeds into the caller's answer.
- **`cap_at`** showed that function parameters are immutable bindings by default, and that adding `mut` to the parameter name affects only the function's own copy.
  The caller's variable is untouched because `i32` is `Copy`.

The central rule is that a function body is a block whose final expression, without a trailing semicolon, becomes the return value.
Parameters are bindings too, so they are immutable unless you add `mut` inside the function.
