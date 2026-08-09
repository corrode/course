# Counting evens with `for` and `continue`

The parameter here is a `&[i32]`, a *slice*: a borrowed view over a sequence of `i32` values that live somewhere else.
We'll spend more time with slices, and the `&` that borrows them, in the borrowing and vectors chapters.
For now, the only thing you need is that a `for` loop walks a slice one element at a time, handing you each number in turn.

You want to count how many of those numbers are even.
A `for` loop over the slice with a counter you bump on every match does the job.

When the current number is odd, you can use `continue` to skip straight to the next one.
You can then increment the counter without nesting that line inside another `if`.
