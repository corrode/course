# Counting digits with `while`

How many digits does a number have? `0` has one digit; everything
else is "divide by 10 and count how many times you can do it before
hitting zero". That's a natural `while` loop: keep going as long as
the number is non-zero, dividing it down each step.

The shape is the inverse of the `for` loop you wrote for `factorial`:
the iteration count isn't known up front, so a `while` reads better
than a `for`.

## Useful from the standard library

- A `let mut n = n;` shadows the parameter with a mutable copy you
  can update inside the loop. Cheap (`u32` is `Copy`) and keeps the
  signature parameter immutable as a habit.
- The `/=` and `+=` operators are the compound-assign cousins of
  `*=`. `n /= 10` divides `n` by 10 in place.
- A bare `while n > 0 { ... }` exits as soon as `n` reaches zero.
  No `break` needed.
- The `n == 0` special case at the start (returning `1`) keeps the
  general loop from giving the wrong answer for `0`. A `match` would
  also work here, but you'll meet `match` properly in chapter 5.
