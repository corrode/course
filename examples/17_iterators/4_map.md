# Transforming with `map`

Now you need to transform every element instead of collapsing the sequence.
Read this pipeline from left to right: take ownership of the vector's items, transform each one, then collect the results into a new vector.

`map` is lazy: it just describes the transformation.
Nothing runs until `collect` (or another consumer) asks for the results.

## Useful from the standard library

- [`Vec::into_iter`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_iter) consumes the vec and yields owned items.
  That's what lets the closure call `.to_lowercase()` on a `String` directly.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) applies a closure to each item and produces a new iterator with the transformed items.
- [`Iterator::collect`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect) turns the pipeline back into a collection.
  The return type (`Vec<String>`) tells `collect` which collection to produce.
- [`str::to_lowercase`](https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase) returns a fresh `String` with the case folded.
