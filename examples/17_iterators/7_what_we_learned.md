# Wrapping up iterators

You collapsed a numeric vector with `sum`, transformed every element with `map`, kept just the matching ones with `filter`, and combined `filter` with `map` to convert borrowed slices into owned strings.

## What we learned

- An iterator is a pipeline: get one with `.iter()` / `.iter_mut()` / `.into_iter()` (or directly from things like `.chars()` and `.lines()`), chain lazy adapters, then finish with a consumer.
- `iter` yields `&T`, `iter_mut` yields `&mut T`, `into_iter` moves out of the collection and yields `T`.
  Pick the one that matches what you intend to do with each item.
- Adapters (`map`, `filter`, `take`, `skip`, ...) describe the pipeline but do nothing on their own.
  The actual work happens when a consumer (`collect`, `sum`, `count`, `for` loop) asks for results.
- `collect` is generic over the target collection.
  The return type (or a turbofish like `.collect::<Vec<_>>()`) tells it what to build.
- `sum` and `product` need to know their output type.
  Annotate the binding or use `.sum::<i32>()` to keep the compiler happy.
- `filter`'s closure always takes `&T`, so on a `&str` iterator you'll see `&&str`.
  Method calls auto-deref, so `.starts_with(...)` Just Works; comparison operators sometimes need an explicit `*`.
- The `|x| ...` syntax you've been seeing is a closure: an anonymous function passed as an argument.
  Iterators are where closures earn their keep.
