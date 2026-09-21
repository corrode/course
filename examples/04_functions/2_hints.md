# Hints

## sum_to

1. Which input already has an answer without making another call? Handle it
   before computing a smaller input, so unsigned subtraction cannot underflow.
2. Suppose the smaller call has returned its sum. Which term is still missing?
   Both branches of an `if` expression must produce the same type.

## cap_at

1. In the reassignment experiment, the compiler complains about assigning to
   `value`. Function parameters are immutable bindings by default, just like
   `let`.
2. Add `mut` to the parameter binding (not the type):
   `fn cap_at(mut value: i32, max: i32) -> i32`.
