# Wrapping Up Iterators

You summed a numeric array with `sum`, transformed every element with `map`,
kept just the matching ones with `filter`, and combined `filter` with `map` to
convert borrowed slices into owned strings.

## What We Learned

- An iterator pipeline starts with `.iter()`, `.iter_mut()`, `.into_iter()`, or
  a method such as `.chars()` or `.lines()`. Lazy adapters describe what should
  happen to each item. A consumer finishes the pipeline by asking for results.
- `iter` yields `&T`, `iter_mut` yields `&mut T`, `into_iter` moves out of the
  collection and yields `T`. Pick the one that matches what you intend to do
  with each item.
- Adapters (`map`, `filter`, `take`, `skip`, ...) describe the pipeline but do
  nothing on their own. The actual work happens when a consumer (`collect`,
  `sum`, `count`, `for` loop) asks for results.
- `collect` is generic over the target collection. The return type (or a
  turbofish like `.collect::<Vec<_>>()`) tells it what to build.
- `sum` needs to know its output type. The function's return type can supply it;
  otherwise annotate the binding or use `.sum::<i32>()`.
- `filter`'s closure always takes `&T`, so on a `&str` iterator you'll see
  `&&str`. Method calls auto-deref, so `.starts_with(...)` works through extra
  references; comparison operators sometimes need an explicit `*`.
- The `|x| ...` syntax you've been seeing is a closure: an anonymous function
  passed as an argument.

- Fallible `sum` stops at the first `Err` and returns it as a value. Unlike `?`,
  it doesn't return from the surrounding function.
- `next` and `take` can consume only part of a pipeline. With `by_ref`, you can
  resume the same iterator afterward; already-consumed items don't run again.
