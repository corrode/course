# Parsing strings into numbers

`str::parse` turns text into the type you ask for.
It returns a `Result` because not every input can, in fact, become the correct type you asked for.

Returning `0` on failure is a *bad idea* in real code because it makes valid input `"0"` indistinguishable from garbage.
In Rust, `Option` and `Result` preserve the distinction between valid zero and invalid input.
For scenarios, where the absence of a value is an error, `Result` is the right choice.
In our case, we have a problem if we try to parse a string that isn't a number. What should be the fallback value?
For this simple case, we will use `0` as the fallback.

Note that `u32` can't be negative, so `"-5".parse::<u32>()` will fail and we should also return `0`.

## Useful from the standard library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) turns a string into a type you choose.
  Returns a `Result` because the input might not be valid.
- [`Result::unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) hands back the value on `Ok`, or the fallback you give it on `Err`.
  Useful for the "just give me a number" path here.
