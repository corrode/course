# Counting evens with `for` and `continue`

The parameter here is a `&[i32]`, a *slice*: a borrowed view over a sequence of `i32` values that live somewhere else.
A `for` loop over the slice gives you a reference to each number in turn.
In `for number in numbers`, `number` has type `&i32`.
You may also see `for &number in numbers`: the `&number` pattern matches the reference and copies the `i32` into `number`.
It doesn't change the slice; integers are `Copy`.
Keep a counter and bump it whenever the number is even.

When the current number is odd, you can use `continue` to skip straight to the next one.
You can then increment the counter without nesting that line inside another `if`.
