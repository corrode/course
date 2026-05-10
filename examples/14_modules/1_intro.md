# Modules and visibility

Modules are how Rust organizes code into namespaces. They give you two
things: a way to group related items together, and a way to control
which of those items are visible from the outside.

The default is private. Add `pub` to expose something:

```rust
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {     // visible outside
        a + b
    }

    fn helper() -> i32 { 42 }               // private to this module
}

fn main() {
    let sum = calculator::add(1, 2);   // OK
    // calculator::helper();           // ERROR: private
}
```

You can declare a module inline (as above) or in a separate file. The
syntax `mod foo;` (no body) tells the compiler to look for `foo.rs` or
`foo/mod.rs` next to the current file.

## Visibility for fields and variants

Marking a `struct` `pub` only makes the *type* public. The fields are
still private unless individually marked. Same for `enum` variants:

```rust
mod config {
    pub struct Settings {
        pub port: u32,        // public field
        secret: String,       // private even though Settings is pub
    }

    impl Settings {
        pub fn new(port: u32) -> Self {
            Settings { port, secret: String::new() }
        }
    }
}
```

This is how you build clean APIs: expose the bare minimum (constructors,
methods, sometimes a few fields), keep everything else private. Callers
can't reach into your internals, so you're free to refactor them later.

## Path syntax

- `crate::foo::bar` is the absolute path from the crate root.
- `super::bar` goes up one module (like `..` in filesystem paths).
- `self::bar` is the current module (rarely needed).
- `use foo::bar;` brings `bar` into scope so you can call it without the
  full path.

## Useful resources

- [The Rust Book on modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
  is the long-form reference, including how packages and crates fit in.
- [The Rust Reference on visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
  is the precise rules, when you need them.
- [`pub(crate)`](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself)
  is a useful middle ground: visible everywhere in your crate, hidden
  from external users.
