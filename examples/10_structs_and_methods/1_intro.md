# Structs and methods

A `struct` groups related fields under a single named type. Once you have
a struct, you can attach methods to it with an `impl` block.

```rust
struct User {
    email: String,
    name: String,
    is_verified: bool,
}

impl User {
    /// Associated function: no `self`. Called as `User::new(...)`.
    fn new(email: String, name: String) -> Self {
        User {
            email,
            name,
            is_verified: false,
        }
    }

    /// Method: takes `&self`, called as `user.display_name()`.
    fn display_name(&self) -> String {
        format!("{} ({})", self.name, self.email)
    }

    /// Mutating method: takes `&mut self`.
    fn verify(&mut self) {
        self.is_verified = true;
    }
}
```

The three flavors of `self` are the heart of methods:

- `&self` reads the struct without modifying it. Most methods.
- `&mut self` modifies the struct in place. Requires the caller to have a
  mutable binding.
- `self` (no reference) consumes the struct, taking ownership. Use this
  when the method returns a transformed version and the original
  shouldn't be reused.

Field access uses dot notation (`user.name`). Inside `impl` you write
`self.field` for the same thing.

`Self` (capital S) is shorthand for "the type I'm `impl`ing". `User` and
`Self` are interchangeable inside `impl User`.

## Useful from the standard library

- [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
  via `#[derive(Debug)]` lets you print a struct with `{:?}` for quick
  inspection.
- [`std::cmp::PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
  via `#[derive(PartialEq)]` enables `==` on structs (compared
  field-by-field).
- [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
  via `#[derive(Default)]` gives you `User::default()` with every field
  set to its type's zero value. Useful for "empty" instances.
- [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
  via `#[derive(Clone)]` adds `.clone()` so you can duplicate the struct
  when you need two owners.
- [The Rust Book on structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
  covers tuple structs, unit structs, and the field shorthand syntax.
