# Parse percentage

A trailing `%` is allowed, but the remaining text may still fail to parse, and a parsed `u8` may be greater than `100`.
Those two failures need different error messages.

The error type here is `&'static str`, so use string literals for the messages.
If you find yourself wanting `format!("{input} is out of range")` in an `Err`, you'd need to change the return type to `Result<u8, String>`.
Stick with literals for this exercise.

## Useful from the standard library

- [`str::strip_suffix`](https://doc.rust-lang.org/std/primitive.str.html#method.strip_suffix) removes a trailing pattern if present and returns `Option<&str>`.
  `input.strip_suffix('%').unwrap_or(input)` peels the `%` when there is one.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) tries to parse the text and returns a `Result`.
  The turbofish (`parse::<u8>()`) tells it which numeric type to produce.
  `u8` already rejects negative numbers and anything above `255`, so those inputs produce parse errors.
- [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) transforms the error value without touching `Ok`.
  You can use it to turn the parser's error into your own static message.
- A bounds check `if n > 100 { return Err("...") }` finishes the job; the `u8` type already takes care of `n < 0`.
