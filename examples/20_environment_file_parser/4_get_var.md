# Typed lookup with generics

Configuration values are stored as strings, but consumers want `u16` ports, `bool` flags, and so on.
Rather than write one helper per type, declare a generic function bounded by `FromStr` and let the caller pick the type at the call site with a turbofish or a type annotation.

Inside the body, `env.get(key)?` short-circuits on a missing key and `.parse().ok()` collapses the parse `Result` into an `Option`.
Don't try to `?` the parse: `T::Err` is unconstrained here and would need an extra `From` bound.

## Useful from the standard library

- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get) returns `Option<&String>`.
  The `?` propagates the missing-key case as `None`.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) uses [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) to produce `Result<T, T::Err>`.
  That's the trait the `where` clause is asking for.
- [`Result::ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok) drops the error and yields `Option<T>`, exactly the function's return type.
- The body fits on one line: `env.get(key)?.parse().ok()`.
