# Creating a config map

The simplest way to use a `HashMap` is to create one and insert a few
key-value pairs. `HashMap::new()` gives you an empty map; the type is
usually inferred from the first `insert`.

Here you'll build a small configuration map with two defaults: a
`"host"` and a `"port"`.

## Useful from the standard library

- [`HashMap::new`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.new)
  creates an empty map. The map needs to be `mut` to insert into it.
- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)
  adds (or replaces) a key-value pair. Both arguments are owned, so
  `&str` literals need a `.to_string()` to become `String`.
