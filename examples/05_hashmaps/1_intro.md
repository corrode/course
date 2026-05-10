# HashMaps

A `HashMap<K, V>` stores key-value pairs and lets you look up a value by
its key in (on average) constant time. It's the workhorse for caches,
indexes, counters, configuration, and anything else where "given X, find
Y" is the question.

`HashMap` lives in `std::collections`, so you need to import it:

```rust
use std::collections::HashMap;

let mut config: HashMap<String, String> = HashMap::new();
config.insert("host".to_string(), "localhost".to_string());

let host = config.get("host"); // Option<&String>
```

Two things to notice:

- Keys and values can be any type, but **all keys share one type and all
  values share one type**. Mix-and-match goes through enums.
- `.get(key)` returns `Option<&V>`, not `V`. Missing keys are explicit, no
  null. Use `.unwrap_or(...)` or pattern matching to handle absence.

A common pattern is "increment a counter for this key, default to 0":

```rust
let mut counts: HashMap<String, u32> = HashMap::new();
for word in ["a", "b", "a"] {
    *counts.entry(word.to_string()).or_insert(0) += 1;
}
```

`entry().or_insert()` is the idiomatic way to do "look up, or insert a
default and then take a mutable reference to it" in one step.

## Useful from the standard library

- [`HashMap::new`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.new)
  creates an empty map. Type is usually inferred from the first `insert`.
- [`HashMap::insert`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.insert)
  adds or updates a key. Returns the previous value, if any.
- [`HashMap::get`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)
  looks up a key and returns `Option<&V>`.
- [`HashMap::contains_key`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key)
  is a quick "is this key present?" without retrieving the value.
- [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
  is the API for "look up; if missing, insert a default; then operate on
  the value." Indispensable for counters and accumulators.
- [`Option::unwrap_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or)
  pulls a value out with a fallback when `None`. Pairs with `.get()`.
