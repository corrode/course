# Transforming with `map`

Now you need to transform every element instead of collapsing the
sequence. The shape is `vec.into_iter()` -> some combinator that
applies a closure -> back to a `Vec` via `collect()`.

`map` is lazy: it just describes the transformation. Nothing runs
until `collect` (or another consumer) asks for the results.
