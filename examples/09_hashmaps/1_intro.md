# HashMaps

A `HashMap<K, V>` stores key-value pairs and lets you look up a value by its key
in (on average) constant time. It's useful for caches, indexes, counters, and
configuration.

Unlike `Vec<T>`, `HashMap` is not in scope by default, so you have to import it
first:

```rust
use std::collections::HashMap;

let mut config: HashMap<String, String> = HashMap::new();
config.insert("host".to_string(), "localhost".to_string());

let host = config.get("host"); // Option<&String>
```

The type annotation says that every key in this map is a `String`, and so is
every value. If you need several possible value types in one map, an enum can
represent those choices.

The `.get(key)` call returns `Option<&V>`, not `V`. A missing key becomes `None`
instead of a null value, so you handle the absence with `.unwrap_or(...)` or
pattern matching.

Use `entry` when a default should fill a gap without replacing an existing
value:

```rust
config.entry("theme".to_string()).or_insert("light".to_string());
```

Unlike `insert`, this leaves an existing theme unchanged. `or_insert` returns a
mutable reference to the value, whether it was already there or just inserted.
You can use that reference to change the stored value without another lookup.

## A Note on `*` (Dereference)

The *dereference operator*, `*`, lets you change a value through a mutable
reference. This works with references returned by `or_insert`, just as it does
with a reference to a local number:

```rust
let mut n = 41;
let r: &mut i32 = &mut n;
*r += 1; // updates `n`, not `r`
```

Without the `*`, you'd be trying to add `1` to a reference, which the compiler
won't let you do. You met references in the
[borrowing chapter](/exercise/borrowing_and_ownership). Here, the practical rule
is that when a function returns `&mut T`, you reach the `T` through `*`.

