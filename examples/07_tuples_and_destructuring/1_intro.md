# Tuples and Destructuring

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

That `min_max` body has three pieces of syntax we haven't formally
introduced yet. Don't let them trip you up here:

- `values.iter()` walks the slice one element at a time. Iterators
  get a full chapter later; for now read it as "give me each
  element in turn."
- `.min()` / `.max()` return an `Option` (they'd return `None` for
  an empty slice). `.unwrap()` says "I'm sure it's `Some`, give me
  the value or panic." `Option` is the next chapter.
- The leading `*` *dereferences* the `&i32` that the iterator hands
  back, so we end up with an owned `i32` instead of a reference.

When you only care about some fields, use `_` to ignore the rest:

```rust
let (first, _) = ("Alice", "Smith");
```

Tuples are great for short-lived "two or three values that belong
together" situations. When the tuple grows or you find yourself passing
it around a lot, that's a hint to define a `struct` instead (chapter 12).


