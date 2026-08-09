# Traits

A **trait** is Rust's word for "a named collection of method signatures that any type can opt into."
If you've used Java or C# interfaces, C++ abstract classes with pure virtual methods, Haskell type classes, Swift protocols, or Python's `abc`/`Protocol`, you already know the gist of it.

The Rust spelling is:

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

`English` and `German` have nothing in common structurally, but both "implement `Greet`."
Anywhere code asks for a `Greet`, either will do.

You've actually been using traits since the enums chapter.
Every time you wrote `#[derive(Debug, PartialEq)]` on an enum or struct, you were asking the compiler to write the `impl Debug for ...` and `impl PartialEq for ...` blocks for you.
That's all `derive` is: a macro that emits the obvious implementation so you don't have to type it out.
We'll revisit this in a moment.

## From familiar traits to trait objects

The first exercise implements `Display`, a standard library trait, for a temperature type.
`Describable` provides a bound for a generic function.
Default methods share behavior between implementations without repetition.
`dyn Trait` lets one collection hold values of different concrete types.

## Standard library traits you've already met

| Trait | What it gives you | Where you know it from |
| --- | --- | --- |
| `Debug` | `{:?}` formatting | enums |
| `Display` | `{}` formatting | the exercises below |
| `PartialEq`, `Eq` | `==` and `!=` | enums |
| `Clone`, `Copy` | `.clone()` and implicit copies | structs and methods |
| `Default` | `T::default()` | earlier mentions |
| `Iterator` | `for x in iter`, all the combinators | iterator pipelines in later exercises |
| `From`, `Into` | `T::from(x)` and `x.into()` conversions | earlier conversions |

None of those are magic.
Each is a regular trait defined in `std`, with implementations for the built-in types where they make sense.
When you `derive` one, the compiler writes the implementation.
When the generated behavior isn't what you want, you write the implementation by hand.

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

You'll use both forms in the exercises below.
For ownership, `Box<dyn Trait>` gives an unsized trait object a fixed-size handle.
