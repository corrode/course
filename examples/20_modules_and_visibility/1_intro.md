# Modules and Visibility

Items in a Rust module are private by default, so outside code can't call your helpers just because it knows their names.
Modules organize code into namespaces and let you choose which items to expose with `pub`:

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

You can declare a module inline (as above) or in a separate file.
At the crate root, `mod foo;` (no body) tells the compiler to look for `foo.rs` or `foo/mod.rs` next to the root file.
Nested modules follow the module hierarchy: `mod bar;` inside `foo.rs` looks for `foo/bar.rs` or `foo/bar/mod.rs`.

## Visibility for Struct Fields

Marking a `struct` `pub` only makes the *type* public.
Its fields are still private unless you mark them individually.
Enums work differently: once an enum is public, its variants are public too.
For a struct, you choose field by field:

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

Start with as little `pub` as possible, so callers don't depend on details you may want to change later.
Expose the constructors, methods, and fields they need, and leave the rest private.

## Path Syntax

- `crate::foo::bar` is the absolute path from the crate root.
- `super::bar` goes up one module (like `..` in filesystem paths).
- `self::bar` is the current module (rarely needed).
- `use foo::bar;` brings `bar` into scope so you can call it without the full path.

## Useful Resources

- [The Rust Book on modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) is the long-form reference, including how packages and crates fit in.
- [The Rust Reference on visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) has the precise rules for when you need them.
- [`pub(crate)`](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself) is a useful middle ground: visible everywhere in your crate, hidden from external users.
