# Defining Your Own Trait

Instead of `impl`ing a trait someone else wrote, you'll write the trait yourself, give two types their own implementation, and then write a *generic* function that accepts anything implementing it.

```rust
trait Describable {
    fn describe(&self) -> String;
}
```

That's the entire interface.
Any type can opt in by writing `impl Describable for MyType { fn describe(&self) -> String { ... } }`.

## Trait Bounds on Generics

Once a trait exists, you can use it as a *bound* on a generic parameter to say "I accept any `T`, as long as `T` implements this trait":

```rust
fn print_one<T: Describable>(item: &T) {
    println!("{}", item.describe());
}
```

This is one way Rust supports polymorphism.
The compiler produces one specialized copy of `print_one` per type you call it with.
That's called *monomorphization*; the C++ template crowd will feel at home.
There's no runtime dispatch and no boxing.

Real code often spells the same kind of bound in one of these forms:

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

For this exercise, use the simple `<T: Describable>` form.
You don't need to memorize all three spellings now; recognize the bound they express.

## Useful from the Standard Library

- [`[String]::join`](https://doc.rust-lang.org/std/primitive.slice.html#method.join) works on a `Vec<String>` too: build a `Vec<String>` of per-item descriptions, then join them with newlines.
- [`Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) can call `describe` on each item; `.collect::<Vec<_>>()` gathers the returned strings.
  A `for` loop with `Vec::push` works too if you prefer to wait for the iterators chapter.
