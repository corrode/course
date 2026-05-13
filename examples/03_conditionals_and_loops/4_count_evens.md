# Counting evens with `for` and `continue`

Now you have a slice of integers and want to know how many of them
are even. The natural shape is a `for` loop over the slice with a
counter that you bump on every match.

This is a good place to use `continue`: skip the odds early and the
"do work" branch ends up uncluttered.

## Hints

- `for n in numbers` over a `&[i32]` yields `&i32` (a reference to
  each element). The `%` operator works through the reference, so
  `n % 2` Just Works without an explicit `*`.
- `continue` skips the rest of the current iteration and starts the
  next one. Combine it with an `if` to short-circuit on the values
  you don't care about.
- `let mut count = 0u32;` is the accumulator. The `0u32` suffix
  tells the compiler exactly which integer type to use without
  needing a separate `: u32` annotation.
