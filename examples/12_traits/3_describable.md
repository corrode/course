# Defining your own trait

Now you're on the other side of the contract. Instead of `impl`ing a
trait someone else wrote, you'll write the trait yourself, give two
types their own implementation, and then write a *generic* function
that accepts anything implementing it.

```rust
trait Describable {
    fn describe(&self) -> String;
}
```

That's the entire interface. Any type can opt in by writing
`impl Describable for MyType { fn describe(&self) -> String { ... } }`.

## Trait bounds on generics

Once a trait exists, you can use it as a *bound* on a generic parameter
to say "I accept any `T`, as long as `T` implements this trait":

```rust
fn print_one<T: Describable>(item: &T) {
    println!("{}", item.describe());
}
```

This is Rust's answer to "polymorphism." The compiler stamps out one
specialized copy of `print_one` per type you call it with (called
*monomorphization*; the C++ template crowd will feel at home). There's
no runtime dispatch and no boxing.

A few variations you'll meet in real code, just so you recognize them:

```rust
// Multiple bounds with `+`:
fn show<T: Describable + std::fmt::Debug>(item: &T) { /* ... */ }

// Same thing, written with a `where` clause. Easier to read once you
// have several parameters or long bounds:
fn show<T>(item: &T)
where
    T: Describable + std::fmt::Debug,
{
    /* ... */
}

// `impl Trait` in argument position is shorthand for a single
// unnamed generic parameter:
fn show(item: &impl Describable) { /* ... */ }
```

You'll write the simple `<T: Describable>` form in this step. The
others are sugar for the same underlying machinery.

## Useful from the standard library

- [The Rust Book on traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
  walks through definitions, implementations, and bounds with more
  examples than fit here.
- `Vec<String>::join("\n")` (and any `&[String].join(...)`) is handy
  for the `print_descriptions` exercise: build a `Vec<String>` of
  per-item descriptions, then join them with newlines.
- The standard `Iterator::map` plus `.collect::<Vec<_>>()` is the
  idiomatic way to turn a `&[T]` into a `Vec<String>`. You met both
  in passing; Chapter 15 covers iterators properly.
