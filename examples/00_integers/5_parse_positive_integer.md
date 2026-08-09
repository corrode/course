# Parsing strings into numbers

`str::parse` turns text into the type you ask for.
It returns a `Result` because not every input can become the type you asked for.
`Result` represents parse success or failure, while this exercise deliberately maps failure to `0`.

Returning `0` on failure is a *bad idea* in real code because it makes valid input `"0"` indistinguishable from garbage.
`Option` and `Result` preserve the distinction between valid zero and invalid input.
The tests define `0` as the fallback.

Note that `u32` can't be negative, so `"-5".parse::<u32>()` will fail and we should also return `0`.
If you reach for `i32` first, the test for `"-5"` may surprise you.

## Useful from the standard library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) turns a string into a type you choose.
  Returns a `Result` because the input might not be valid.
- [`Result::unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) hands back the value on `Ok`, or the fallback you give it on `Err`.
  Useful for the "just give me a number" path here.
