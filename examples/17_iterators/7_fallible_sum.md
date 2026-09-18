# Revisit: summing parsed numbers

In the `?` chapter, you parsed each token inside a `for` loop.
Write `sum_numbers` again, this time with an iterator pipeline and `sum`, without a loop or `?`.
Keep the same behavior: whitespace separates integers, empty input returns `Ok(0)`, and the first parse error ends the sum.
Assume the running total fits in an `i32`.

[`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) also works on an iterator of `Result` values.
Its output can be a `Result` containing either the total or the first error.
The function's return type can tell it which output type to use.

Compare the two versions after the tests pass.
In the loop, `?` returns from `sum_numbers` when parsing fails.
Here, `sum` stops asking for items and returns an `Err` to its caller; it does not return from the surrounding function.
Your function can inspect that result or return it directly.
Adding `?` after `sum` would propagate an error that `sum` has already found.
