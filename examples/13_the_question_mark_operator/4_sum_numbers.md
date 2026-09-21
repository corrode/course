# `?` inside a Loop

`add_parsed_numbers` had two chances to return a parse error. `sum_numbers` may
inspect many tokens, but it still returns only the first parse error it
encounters.

`sum_numbers` takes text with integers separated by whitespace and adds them up.
The first token that isn't a number makes the function return that
`ParseIntError` and stop. Since the function only parses, one error type covers
every failure without boxing or conversion.

Use a `for` loop and apply `?` to each token's parse result. What happens to the
rest of the loop when parsing fails? Return `Ok(0)` for empty or whitespace-only
input. Assume the running total fits in an `i32`.

## Useful from the Standard Library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
  yields each token as a `&str`, skipping the gaps between numbers.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  returns a `Result`; `parse::<i32>()` asks for an integer.
