# Wrapping up functions

Three small exercises, three new ideas:

- **`stray_semicolon`** drove home the difference between an expression and a statement: one trailing `;` is the line between "return this value" and "return `()`".
- **`sum_to`** built a value with recursion, where each call's answer feeds into the caller's answer.
- **`cap_at`** showed that function parameters are immutable bindings by default, and that adding `mut` to the parameter name affects only the function's own copy.
  The caller's variable is untouched because `i32` is `Copy`.

The thread running through all of this: the body of a function is a block whose final expression (no trailing semicolon) is the return value.
Everything else in the chapter is a consequence of that one rule.
