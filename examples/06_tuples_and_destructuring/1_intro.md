# Tuples and destructuring

A tuple is a fixed-size group of values. Unlike a `Vec`, the elements can
be different types, and the size is part of the type.

```rust
let user: (String, u32) = ("Alice".to_string(), 25);
let pair = (1, 2);                 // type inferred as (i32, i32)
let triple = ("ok", 200, true);    // (&str, i32, bool)
```

You access fields by index with a dot:

```rust
let name = user.0;
let age = user.1;
```

But the more idiomatic way is *destructuring*: pull the parts out into
named bindings in one step.

```rust
let (name, age) = user;
let (a, b) = (1, 2);

// Functions can return tuples for multiple values:
fn min_max(values: &[i32]) -> (i32, i32) {
    (*values.iter().min().unwrap(), *values.iter().max().unwrap())
}
let (lo, hi) = min_max(&[3, 1, 4, 1, 5, 9]);
```

When you only care about some fields, use `_` to ignore the rest:

```rust
let (first, _) = ("Alice", "Smith");
```

Tuples are great for short-lived "two or three values that belong
together" situations. When the tuple grows or you find yourself passing
it around a lot, that's a hint to define a `struct` instead (chapter 10).

## Useful from the standard library

- [The Rust Book on tuples](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
  for the basics of declaration and access.
- [Rust by Example: destructuring tuples](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
  shows how patterns work inside `match` arms too.
- [`std::mem::swap`](https://doc.rust-lang.org/std/mem/fn.swap.html)
  swaps two values without a temporary. Worth knowing about, though
  destructuring + a re-binding is usually cleaner.
- The unit type [`()`](https://doc.rust-lang.org/std/primitive.unit.html)
  is the empty tuple. Functions that return "nothing meaningful" actually
  return `()`.
