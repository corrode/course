# Tuples and Destructuring

Sometimes a function needs to return two things. 
Coming up with a type name for that is not always worth it. 

Rust has support for tuples, which are fixed-size groups of values. Unlike a
`Vec`, the elements can be different types, and the size is part of the type.

```rust
let response: (u16, &str, bool) = (200, "ok", true);
let pair = (1, 2); // type inferred as (i32, i32)
```

You access fields by index with a dot:

```rust
if response.2 {
    println!("cached response: {}", response.0);
}
```

I prefer *destructuring* here so I can read `status` and `cached` rather than
remember what `.0` and `.2` mean. It pulls the parts out into named bindings in
one step.

```rust
let (status, message, cached) = response;
if cached {
    println!("cached response: {status} {message}");
}

// Functions can return tuples for multiple values:
fn min_max(values: &[i32]) -> (i32, i32) {
    (*values.iter().min().unwrap(), *values.iter().max().unwrap())
}
let (lo, hi) = min_max(&[3, 1, 4, 1, 5, 9]);
```

You only need a rough reading of the `min_max` body for now. These details are
enough to follow the example:

- `values.iter()` walks the slice one element at a time. For now, read it as
  "give me each element in turn."
- `.min()` / `.max()` return an `Option` (they'd return `None` for an empty
  slice). `.unwrap()` says "I'm sure it's `Some`, give me the value or panic."
- The leading `*` *dereferences* the `&i32` the iterator hands back (the same
  dereference you met in the hashmaps chapter), so we end up with an owned `i32`
  instead of a reference.

When you only care about some fields, use `_` to ignore the rest:

```rust
let (_, _, cached) = response;
```

Tuples are great for short-lived "two or three values that belong together"
situations. When the tuple grows or you keep passing the same tuple type around,
give those fields names with a `struct` instead.

