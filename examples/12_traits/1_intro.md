# Traits

A **trait** is Rust's word for "a named collection of method signatures
that any type can opt into." If you've used Java or C# interfaces, C++
abstract classes with pure virtual methods, Haskell type classes, Swift
protocols, or Python's `abc`/`Protocol`, you already know the shape of
the idea. The Rust spelling is:

```rust
trait Greet {
    fn hello(&self) -> String;
}

struct English;
struct German;

impl Greet for English {
    fn hello(&self) -> String { "Hello!".to_string() }
}

impl Greet for German {
    fn hello(&self) -> String { "Hallo!".to_string() }
}
```

`English` and `German` have nothing in common structurally, but both
"implement `Greet`." Anywhere code asks for a `Greet`, either will do.

You've actually been using traits since Chapter 5. Every time you wrote
`#[derive(Debug, PartialEq)]` on an enum or struct, you were asking the
compiler to write the `impl Debug for ...` and `impl PartialEq for ...`
blocks for you. That's all `derive` is: a macro that emits the obvious
implementation so you don't have to type it out. We'll revisit this in
a moment.

## What's in this chapter

1. **Implementing an existing trait.** You'll write `impl Display for
   Temperature` so a value formats itself as `"21.5°C"` in `println!`
   and `format!`.
2. **Defining your own trait.** A `Describable` trait with two
   implementations, plus a generic function with a *trait bound* that
   accepts any `T: Describable`.
3. **Default methods.** Traits can ship a default body for a method,
   which implementors inherit unless they override it. This is where
   traits start to feel like mixins.
4. **Trait objects (`dyn Trait`).** Generics give you one specialized
   copy of a function per concrete type ("static dispatch"). Trait
   objects give you one function that dispatches at runtime ("dynamic
   dispatch") and let you put *different* types into the same `Vec`.

## A quick map of stdlib traits you've already met

| Trait | What it gives you | First seen |
| --- | --- | --- |
| `Debug` | `{:?}` formatting | Chapter 5 (enums) |
| `Display` | `{}` formatting | this chapter |
| `PartialEq`, `Eq` | `==` and `!=` | Chapter 5 |
| `Clone`, `Copy` | `.clone()` and implicit copies | Chapter 10 |
| `Default` | `T::default()` | mentioned in passing |
| `Iterator` | `for x in iter`, all the combinators | Chapter 15 |
| `From`, `Into` | `T::from(x)` and `x.into()` conversions | sprinkled throughout |

None of those are magic. Each is a regular trait defined in `std`, and
the standard library `impl`s it for the obvious built-in types. When
you `derive` one, the compiler writes the impl. When you can't derive
(maybe the auto-generated version isn't what you want), you write
`impl Display for MyType { ... }` by hand, just like in step 2.

## Static vs. dynamic dispatch: a sneak preview

```rust
// Static dispatch: the compiler generates a specialized copy of
// `print_all` for each `T` you use it with. Zero runtime cost,
// but every `T` in one call must be the same concrete type.
fn print_all<T: Display>(items: &[T]) { /* ... */ }

// Dynamic dispatch: one function, one vtable lookup per call.
// The slice can mix different concrete types that all implement
// `Display`.
fn print_all_dyn(items: &[&dyn Display]) { /* ... */ }
```

You'll write both in this chapter. The `Box<dyn Trait>` form, which
solves the "but trait objects don't have a known size" problem you'll
bump into, is the headline act of Chapter 14 on smart pointers.
