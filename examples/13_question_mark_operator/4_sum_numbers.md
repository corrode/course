# `?` through an iterator

`add_parsed_numbers` propagated a single parse error.
This step propagates a whole list of them.

`sum_numbers` takes text with integers separated by whitespace and adds them up.
The first token that isn't a number makes the function return that `ParseIntError` and stop.
Because the function only parses (no file reading), one error type covers it: no boxing, no conversion.

The interesting part is that `?` rides straight through an iterator pipeline.

## Useful from the standard library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) yields each token as a `&str`, skipping the gaps between numbers.
- `.map(|token| token.parse::<i32>())` turns each token into a `Result<i32, ParseIntError>`.
- [`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) has an impl that adds a sequence of `Result`s: it returns the first `Err`, or the total wrapped in `Ok`.
  So `.sum::<Result<i32, _>>()?` collapses the whole list to an `i32`, or short-circuits on the first bad token.
