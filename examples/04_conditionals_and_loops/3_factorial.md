# Factorial with a `for` loop

`n!` is `1 * 2 * 3 * ... * n`. By convention, `0! == 1`. Build it
up with a running accumulator and a `for` loop over an inclusive
range.

The accumulator pattern shows up everywhere once you start writing
loops: `let mut acc = ...; for x in ... { acc = ... }; acc`. Note
the `mut`: bindings are immutable by default, and the loop body
needs to update `acc`, so you have to opt in.

## Useful from the standard library

- `let mut acc: u32 = 1;` declares a mutable binding. Without
  `mut`, the compiler refuses any reassignment or compound-assign
  like `*=`.
- `1..=n` is the inclusive range from `1` to `n`. For `n == 0` it's
  empty, so the accumulator stays at its initial `1`. That's why
  `factorial(0)` returns `1` for free, no special case needed.
- The `*=` operator multiplies the left-hand side by the right and
  stores it back. `acc *= i;` is the standard accumulator step.
