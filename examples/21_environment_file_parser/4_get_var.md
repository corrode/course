# Typed lookup with generics

Configuration values are stored as strings, but consumers want `u16` ports, `bool` flags, and so on.
Rather than write one helper per type, declare a generic function bounded by `FromStr` and let the caller pick the type at the call site with a turbofish or a type annotation.

Return `None` both when the key is missing and when its value cannot be parsed as the requested type.
The standard library methods below provide the pieces; your task is to combine them.

## Useful from the standard library

- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get) returns `Option<&String>`.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) uses [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) to produce `Result<T, T::Err>`.
  That's the trait the `where` clause is asking for.
- [`Result::ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok) drops the error and yields `Option<T>`, matching the function's return type.
