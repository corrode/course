# Destructuring a tuple parameter

You can destructure a tuple right in the function parameter list, or
inside the body with a `let` binding. Either way, you pull out the
pieces by position.

Watch out for ownership: a tuple of `String`s is *moved* into the
function, while a tuple of integers is *copied*. "Moved" means the
caller's binding is no longer usable afterwards, because the value's
single owner is now the function parameter rather than the caller. "Copied" means
the value is duplicated bit-for-bit, so the caller keeps theirs and
the function gets its own. The split is decided by a trait called
`Copy`: types that are tiny and have no heap data (integers, bools,
`char`, fixed-size arrays of those, and tuples made entirely of
`Copy` types) implement it; types that own heap data (like `String`
or `Vec`) deliberately don't. The doc-comment below has more on
this, and chapter 11 covers move semantics in depth.

## Useful from the standard library

- [Rust by Example: destructuring tuples](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_tuple.html)
  shows the `let (a, b) = pair;` form and how `_` can ignore parts
  you don't want to bind.
- Field-by-index access (`full_name.0`) also works, but a destructure
  with a meaningful name like `first` reads better at the call site.
- Anything that isn't `Copy` (like `String`) is *moved* when bound
  by destructuring, so the caller's binding becomes unusable: ownership
  of the underlying heap buffer transferred into the function. `Copy`
  types (integers, bools, `char`, and tuples of those) are duplicated
  instead, so the caller keeps their copy. After this function
  returns, the caller's `(String, String)` tuple is gone. Chapter 11
  covers move semantics in depth.
