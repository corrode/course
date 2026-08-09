# `?` through an iterator

`add_parsed_numbers` had two chances to return a parse error.
`sum_numbers` may inspect many tokens, but it still returns only the first parse error it encounters.

`sum_numbers` takes text with integers separated by whitespace and adds them up.
The first token that isn't a number makes the function return that `ParseIntError` and stop.
Since the function only parses, one error type covers every failure without boxing or conversion.

Here the iterator pipeline produces one `Result`, and `?` either unwraps its total or returns the first error.

## Useful from the standard library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) yields each token as a `&str`, skipping the gaps between numbers.
- Parsing each token turns the iterator into a sequence of `Result<i32, ParseIntError>` values.
- [`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) can add that sequence of `Result`s, returning the first `Err` or the total wrapped in `Ok`.
  Once `sum` has collapsed those results, `?` gives you the total on success.
