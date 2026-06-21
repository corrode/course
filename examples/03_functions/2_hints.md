# Hints

## sum_to

1. The body is an `if` with a base case (`n == 0`) and a recursive case
   that calls `sum_to(n - 1)`.
2. The recursive case returns `n + sum_to(n - 1)`. No `mut`, no `let`,
   no `return`.

## cap_at

1. The compiler complains about assigning to `value`. Function
   parameters are immutable bindings by default, just like `let`.
2. Add `mut` to the parameter binding (not the type):
   `fn cap_at(mut value: i32, max: i32) -> i32`.
