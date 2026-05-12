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

## Useful from the standard library

- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
  transforms each item.
- [`Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)
  keeps items matching a predicate. The closure takes a `&T` even on a
  `T`-yielding iterator, which is why you sometimes see `**c == ...`.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)
  turns the pipeline back into a collection.
- [`Iterator::sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum)
  and [`Iterator::product`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)
  reduce numeric iterators in one call.
- [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)
  and [`Iterator::all`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all)
  return a `bool`. Short-circuit, so cheap on long iterators.
- [`Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
  returns the first matching item as an `Option`.
- [`Iterator::cloned`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned)
  turns an iterator of `&T` into one of `T` by cloning. Often the easiest
  way to get owned values out of a `.iter()` chain.
- [`Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)
  is the general "accumulate a value across the iterator". Reach for it
  when there's no specific reducer.
