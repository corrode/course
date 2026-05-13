# Parse percentage

This is the hardest function in the chapter; the previous three were
warmups. More than one thing can go wrong, and they need different
error messages. Strip the optional `%` first, then `parse::<u8>()`
the rest, then bounds-check. Each step is its own potential `Err`.

Note: the error type here is `&'static str`, which means the message
has to be a string literal. If you find yourself wanting
`format!("{input} is out of range")` in an `Err`, you'd need to
change the return type to `Result<u8, String>`. Stick with literals
for this exercise.

## Useful from the standard library

- [`str::strip_suffix`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_suffix)
  removes a trailing pattern if present and returns
  `Option<&str>`. `input.strip_suffix('%').unwrap_or(input)` peels
  the `%` when there is one.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  is the canonical "may fail to parse" call. The turbofish
  (`parse::<u8>()`) tells it which numeric type to produce. `u8`
  already rejects negative numbers and anything above `255`, which
  catches a couple of cases for free.
- [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)
  swaps the error type without touching `Ok`. Handy to turn the
  parser's error into your own static message.
- A bounds check `if n > 100 { return Err("...") }` finishes the
  job; the `u8` type already takes care of `n < 0`.
