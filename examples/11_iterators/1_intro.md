# Iterators

*Rust iterators are the best thing since `&[bread]`.*

Iterators turn "loop over a collection and do something" into a pipeline.
They're lazy: nothing actually runs until you ask for a result. The
compiler usually fuses chained iterator calls into a single tight loop,
so the abstraction is free at runtime.

The shape is always the same:

1. Get an iterator with `.iter()`, `.into_iter()`, `.iter_mut()`, or
   directly from `.chars()`, `.lines()`, etc.
2. Chain *adapters* like `.map(...)`, `.filter(...)`, `.take(...)`. These
   are lazy.
3. Finish with a *consumer* like `.collect()`, `.sum()`, `.count()`,
   `.any(...)`, or a `for` loop.

```rust
let names = vec!["alice", "ADMIN", "bob"];

let active: Vec<String> = names
    .iter()                              // &&str
    .filter(|n| n.starts_with('a'))      // keep some
    .map(|n| n.to_lowercase())           // transform
    .collect();                          // back to Vec<String>
// active == ["alice"]
```

The three "iter" methods differ in what they yield:

- `.iter()` yields `&T` (immutable references). Use when reading.
- `.iter_mut()` yields `&mut T`. Use when modifying in place.
- `.into_iter()` yields `T` (consumes the collection). Use when you don't
  need the original anymore.

Some adapters change the item type. After `.map(|n| n.to_lowercase())`,
the items are `String`s, not `&&str`s. The compiler infers types through
the chain, so trust it: write the chain, then add a type annotation on
the binding if needed.

`.collect()` is interesting: it can produce many different collections.
Tell it which one with a type annotation: `Vec<_>`, `HashMap<_, _>`,
`String`. The `_` lets the compiler fill in the inner types.


