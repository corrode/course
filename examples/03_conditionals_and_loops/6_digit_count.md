# Counting Digits with `while`

How many digits does a number have?
`0` has one digit; everything else is "divide by 10 and count how many times you can do it before hitting zero".
That's a natural `while` loop: keep going as long as the number is non-zero, dividing it down each step.

Compare this with the `for` loop you wrote for `factorial`.
With `factorial`, you knew up front how many times to loop.
Here, you keep dividing until the number reaches zero.
That's exactly what `while` is for.
