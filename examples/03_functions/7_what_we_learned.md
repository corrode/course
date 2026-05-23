# Wrapping up functions

Four small exercises, four new ideas:

- **`stray_semicolon`** drove home the difference between an
  expression and a statement: one trailing `;` is the line between
  "return this value" and "return `()`".
- **`countdown`** introduced the unit return type `()` and the basic
  form of a recursive function: a base case plus a recursive case.
- **`sum_to`** turned that into a value-returning recursion,
  where each call's answer feeds into the caller's answer.
- **`cap_at`** showed that function parameters are immutable
  bindings by default, and that adding `mut` to the parameter name
  affects only the function's local copy, not the caller's variable.

The thread running through all of this: the body of a function is a
block whose final expression (no trailing semicolon) is the return
value. Everything else in the chapter is a consequence of that one
rule.
