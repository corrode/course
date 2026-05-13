# Defining a struct and a constructor

A `struct` groups related fields under one name. Rust has no
built-in constructors; the convention is an associated function
called `new` that returns `Self`. "Associated" means it lives in the
`impl` block but doesn't take `self`. You call it as `User::new(..)`.

Here we model a `User` and write the constructor that establishes
the sensible starting state: a brand-new account is unverified and
has zero logins recorded.

## Useful from the standard library

- [The Rust Book on structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
  covers struct literal syntax, the field shorthand
  (`User { email, name, .. }`), and tuple/unit structs.
- [`#[derive(Debug)]`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
  and [`#[derive(PartialEq)]`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
  are already on the struct: that's what makes the test's
  `assert_eq!` work and what would let you `println!("{user:?}")`.
- `Self` (capital S) is interchangeable with the struct's name
  inside an `impl` block. Returning `Self` keeps the constructor
  signature stable if you ever rename the type.
