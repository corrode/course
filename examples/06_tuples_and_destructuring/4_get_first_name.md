# Destructuring a tuple parameter

You can destructure a tuple right in the function parameter list, or
inside the body with a `let` binding. Either way, you pull out the
pieces by position.

Watch out for ownership: a tuple of `String`s is moved into the
function, while a tuple of integers is copied. The doc-comment below
has more on this.

## Useful from the standard library

- [Rust by Example: destructuring tuples](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
  shows the `let (a, b) = pair;` form and how `_` can ignore parts
  you don't want to bind.
- Field-by-index access (`full_name.0`) also works, but a destructure
  with a meaningful name like `first` reads better at the call site.
- Anything that isn't `Copy` (like `String`) is moved when bound by
  destructuring. After this function returns, the caller's tuple is
  gone. Chapter 9 covers move semantics in depth.
