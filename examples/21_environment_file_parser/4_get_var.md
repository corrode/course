# Typed Lookup with Generics

Your map stores strings, but you need a `u16` for a port or a `bool` for a flag.
Rather than write one helper per type, declare a generic function with a `FromStr` bound.
The caller picks the type with a turbofish or a type annotation.

Return `None` both when the key is missing and when its value cannot be parsed as the requested type.
The standard library methods below provide the pieces; your task is to combine them.

## Useful from the Standard Library

- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get) returns `Option<&String>`.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) uses [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) to produce `Result<T, T::Err>`.
  That's the trait the `where` clause is asking for.
- [`Result::ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok) drops the error and yields `Option<T>`, matching the function's return type.

Predict what happens when the stored text is `"256"` and the caller asks for
`u8`, `u16`, or `String`. Also test a missing key and a present but invalid value.
This API deliberately returns `None` for both failures. If a caller needed to
explain the difference, what information would the return type need to preserve?
