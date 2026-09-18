# Wrapping up functions

Here's what each exercise asked you to work out:

- In `stray_semicolon`, one trailing `;` changed an expression into a statement, turning "return this value" into "return `()`".
- In `sum_to`, you built a value with recursion, where each call's answer feeds into the caller's answer.
- In `cap_at`, you saw that function parameters are immutable bindings by default.
  Adding `mut` to the parameter name lets the function change its own copy.
  The caller's variable is untouched because `i32` is `Copy`.

The central rule is that a function body is a block whose final expression, without a trailing semicolon, becomes the return value.
Parameters are bindings too, so they are immutable unless you add `mut` inside the function.
