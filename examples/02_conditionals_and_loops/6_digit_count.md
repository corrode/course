# Counting digits with `while`

How many digits does a number have? `0` has one digit; everything
else is "divide by 10 and count how many times you can do it before
hitting zero". That's a natural `while` loop: keep going as long as
the number is non-zero, dividing it down each step.

The shape is the inverse of the `for` loop you wrote for `factorial`:
the iteration count isn't known up front, so a `while` reads better
than a `for`.
