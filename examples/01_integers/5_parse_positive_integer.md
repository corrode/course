# Parsing strings into numbers

`str::parse` is the universal "turn this text into a value of some
type" method. It returns a `Result`, because the input might not be
parseable. We don't have `Result` yet (that's chapter 8), so we'll
collapse failure to `0` for now.

Returning `0` on failure is a *bad idea* in real code: it silently
merges "the input was the number zero" with "the input was garbage".
Rust has a much better tool for this in `Option` (chapter 7) and
`Result` (chapter 8). For now, `parse().unwrap_or(0)` is the
shortest way to satisfy the tests.

Note that `u32` can't be negative, so `"-5".parse::<u32>()` will
fail and we should also return `0`. If you reach for `i32` first,
the test for `"-5"` may surprise you.

## Useful from the standard library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  turns a string into any type that implements `FromStr`. Returns a
  `Result` because the input might not be valid.
- [`Result::unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or)
  hands back the value on `Ok`, or the fallback you give it on `Err`.
  Useful for the "just give me a number" path here.
