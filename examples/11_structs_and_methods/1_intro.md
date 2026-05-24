# Structs and Methods

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

## A note on ranges: `0..5`

One of the tests in this chapter calls `record_login()` five times in a
loop:

```rust
for _ in 0..5 {
    user.record_login();
}
```

`0..5` is a *range expression*. It produces the values `0, 1, 2, 3, 4`
(end-exclusive). `0..=5` is the inclusive variant if you want `5` too.
The loop variable is `_` here because the body doesn't need it; we
just want "do this thing five times." Ranges are useful as iterators
but also work as slice indices (`v[0..3]`).

> [!TIP]
> From this chapter onward the files get longer, and the in-browser
> editor starts feeling cramped. The **Open in Web Editor** button above
> each editor opens this file on [github.dev](https://github.dev), a
> full browser-based VS Code with proper find-in-file, multi-cursor,
> and the rest of the keyboard shortcuts you'd expect. Clone the repo
> locally if you want `rust-analyzer` and on-save formatting.


