# Getting a value with a fallback

Looking up a key in a `HashMap` returns an `Option<&V>`, because the key might not be there.
There's no null.
To collapse the `Option` into a concrete value you supply a fallback for the missing case.
We'll cover `Option` properly in the `Option` chapter; here you only need one combinator.

## Useful from the standard library

- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)
  returns `Option<&V>`.
  The `&` matters: you get a reference into the map, not a copy of the value.
- [`Option::cloned`](https://doc.rust-lang.org/std/option/enum.Option.html#method.cloned)
  turns `Option<&String>` into `Option<String>` so you can return an owned value.
- [`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
  pulls a value out, falling back to the argument when the option is `None`.
  Reads as "use this value, or default to that one".
