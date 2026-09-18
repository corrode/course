# Parsing Strings into Numbers

`str::parse` turns text into the type you ask for.
It returns a `Result` because the input might not be valid for that type.

Returning `0` on failure is a *bad idea* in real code because it makes valid input `"0"` indistinguishable from garbage.
In Rust, `Option` and `Result` preserve the distinction between valid zero and invalid input.
Use `Result` when a missing value is an error you need to explain.
Here, I'll keep error handling out of the exercise: return `0` if the string isn't a valid number.

`u32` can't be negative, so `"-5".parse::<u32>()` fails and you should return `0` for it too.

## Useful from the Standard Library

- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) turns a string into a type you choose.
  It returns a `Result` because the input might not be valid.
- [`Result::unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or) hands back the value on `Ok`, or the fallback you give it on `Err`.
  Useful for the "just give me a number" path here.
