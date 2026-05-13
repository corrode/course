# Wrapping up structs and methods

You defined a struct, wrote a `new` constructor, added a `&self`
method that formatted fields into a `String`, mutated state with
`&mut self`, and combined two fields into a predicate.

## What we learned

- A `struct` groups related fields under a single named type. Build
  instances with the literal syntax `User { email, name, .. }` and
  read fields with dot notation.
- An `impl` block attaches functions to the type. Without `self`,
  it's an associated function (called as `User::new(..)`); with
  `self`, it's a method (called as `user.method()`).
- The three flavors of `self` say what the method intends to do:
  `&self` reads, `&mut self` mutates in place, plain `self`
  consumes. The same ownership rules from chapter 11 apply.
- `Self` (capital S) inside an `impl` block is shorthand for the
  type. Returning `Self` keeps the constructor signature stable if
  the type is later renamed.
- `format!` is the idiomatic way to build a `String` from a
  template; same syntax as `println!` but returns the string.
- `#[derive(Debug, PartialEq)]` covers the common pair: `{:?}`
  printing for debugging and `==` for tests. Reach for `Default`,
  `Clone`, and `Copy` when they fit.
- Encoding business rules as predicates on the type
  (`user.can_access_premium()`) keeps the rule in one place and
  makes call sites self-documenting.
