# Hints

## countdown

1. The body fits in one `if`/`else`. One branch is the base case,
   the other is the recursive case.
2. The base case is `n == 0` and just prints `"Liftoff!"`. The
   recursive case prints `n` and calls `countdown(n - 1)`.

## sum_to

1. Same recursion pattern as `countdown`: an `if` with a base case
   (`n == 0`) and a recursive case that calls `sum_to(n - 1)`.
2. The recursive case returns `n + sum_to(n - 1)`. No `mut`, no
   `let`, no `return`.

## cap_at

1. The compiler complains about assigning to `value`. Function
   parameters are immutable bindings by default, just like `let`.
2. Add `mut` to the parameter binding (not the type):
   `fn cap_at(mut value: i32, max: i32) -> i32`.
