# Swapping with destructuring

Tuple destructuring makes swapping two values a one-liner: bind the pair to `(a, b)` and return `(b, a)`.
You don't need a temporary variable.

## Useful from the standard library

- [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html) swaps two `&mut T` references in place.
  You can use it when you can't take ownership; here, returning a fresh tuple is simpler.
- The integers in this exercise are `Copy`, so `(b, a)` makes bit-wise copies of both.
  You aren't moving either value.
