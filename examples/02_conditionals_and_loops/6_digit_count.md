# Counting digits with `while`

How many digits does a number have? `0` has one digit; everything
else is "divide by 10 and count how many times you can do it before
hitting zero". That's a natural `while` loop: keep going as long as
the number is non-zero, dividing it down each step.

This is the inverse of a `for` loop (like the one you wrote for `factorial`).
With `factorial`, you knew up front how many times to loop. Here, you don't:
you have to keep dividing until the number runs out. That's exactly what
`while` is for.
