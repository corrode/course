# Counting evens with `for` and `continue`

The parameter here is a `&[i32]`, a *slice*: a borrowed view over a sequence of `i32` values that live somewhere else.
For now, the only thing you need is that a `for` loop walks a slice one element at a time, handing you each number in turn.

A `for` loop gives you each number in turn.
Keep a counter and bump it whenever the number is even.

When the current number is odd, you can use `continue` to skip straight to the next one.
You can then increment the counter without nesting that line inside another `if`.
