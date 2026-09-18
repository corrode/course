# Defining a struct and a constructor

A `struct` groups related fields under one name.
Rust has no built-in constructors; the convention is an associated function called `new` that returns `Self`.
"Associated" means it lives in the `impl` block but doesn't take `self`.
You call it as `User::new(..)`.

You'll write a constructor for `User` so every new account starts unverified, with zero logins recorded.

## Constructor syntax

- [The Rust Book on structs](https://doc.rust-lang.org/book/ch05-00-structs.html) covers struct literals and field shorthand: write `email` instead of `email: email` when the names match.
- `Self` (capital S) is interchangeable with the struct's name inside an `impl` block.
  Returning `Self` keeps the constructor signature stable if you ever rename the type.
