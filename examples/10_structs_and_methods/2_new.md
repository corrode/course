# Defining a struct and a constructor

A `struct` groups related fields under one name. Rust has no
built-in constructors; the convention is an associated function
called `new` that returns `Self`. "Associated" means it lives in the
`impl` block but doesn't take `self`. You call it as `User::new(..)`.

Here we model a `User` and write the constructor that establishes
the sensible starting state: a brand-new account is unverified and
has zero logins recorded.
