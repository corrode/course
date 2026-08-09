# Destructuring a tuple parameter

You can destructure a tuple right in the function parameter list, or inside the body with a `let` binding.
Either way, you pull out the pieces by position.

Ownership still applies when you destructure.
Passing a tuple of `String`s by value moves the whole tuple into the function, so the caller cannot use it afterward.
A tuple of integers is `Copy`, which gives the function its own copy and leaves the caller's value usable.
This is the same move-versus-copy distinction you worked through in the moves chapter.

## Useful from the standard library

- [Rust by Example: destructuring tuples](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
  shows the `let (a, b) = pair;` form and how `_` can ignore parts you don't want to bind.
- Field-by-index access (`full_name.0`) also works, but a destructure with a meaningful name like `first` reads better at the call site.
- Anything that isn't `Copy`, such as `String`, moves when destructured by value.
  A tuple is only `Copy` when all of its elements are `Copy`.
