# A boolean predicate

A function that returns `bool` is called a *predicate*. The body
here is one expression: take the input modulo 2 and check whether
the result is `0`.

The `%` operator is the remainder. For non-negative integers it
behaves exactly the way you'd expect.

## Useful from the standard library

- The `%` operator on integers gives the remainder. `n % 2` is `0`
  for even numbers and `1` for odd ones.
- The `==` operator compares two values for equality and returns
  `bool`. Combined: `n % 2 == 0` is the whole body.
- [`u32::is_multiple_of`](https://doc.rust-lang.org/std/primitive.u32.html#method.is_multiple_of)
  exists in newer Rust versions. The `%` form is the classic and
  works on every supported version.
