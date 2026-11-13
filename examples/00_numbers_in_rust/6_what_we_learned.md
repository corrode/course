# Wrapping Up Numbers

You chose how to handle overflow, converted numeric types explicitly, and used a
fallback when parsing failed.

## What We Learned

- Call a function with `function(arguments)` and a method with
  `value.method(arguments)`. For example,
  [`number.to_string()`](https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string)
  turns a number into a `String`.
- If you add two numbers and the result doesn't fit the type, Rust panics in
  debug builds and wraps in release builds. When overflow is possible, pick the
  behavior you want:
  [`saturating_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add)
  (clamp),
  [`checked_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.checked_add)
  (return an `Option`), or
  [`wrapping_add`](https://doc.rust-lang.org/std/primitive.u8.html#method.wrapping_add)
  (wrap around).
- Rust never mixes numeric types for you. If the types don't match, you must
  convert them explicitly. `as` casts may lose information;
  [`.into()`](https://doc.rust-lang.org/std/convert/trait.Into.html#tymethod.into)
  handles infallible conversions, while
  [`.try_into()`](https://doc.rust-lang.org/std/convert/trait.TryInto.html#tymethod.try_into)
  returns a `Result` when a conversion can fail.
- `as u32` on a float truncates toward zero (`1.7 as u32` is `1`);
  [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)
  rounds to the nearest integer. We used truncation for the damage example.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  turns text into a value of the type you ask for. It returns a `Result`, and
  [`.unwrap_or(...)`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or)
  supplies a fallback when parsing fails.
