# Traits

A trait lets you give unrelated types a shared interface without putting them in a class hierarchy.
You declare a named collection of method signatures, and each type can opt in by implementing it.
If you've used Java or C# interfaces, C++ abstract classes with pure virtual methods, Haskell type classes, Swift protocols, or Python's `abc`/`Protocol`, you already know the gist of it.

The Rust flavor is:

```rust
trait Greet {
    fn hello(&self) -> String;
}

struct English;

impl Greet for English {
    fn hello(&self) -> String { "Hello!".to_string() }
}

struct German;

impl Greet for German {
    fn hello(&self) -> String { "Hallo!".to_string() }
}
```

`English` and `German` have nothing in common structurally, but both "implement `Greet`."
Code written against the `Greet` interface can work with either type.

## Standard Library Traits You've Already Met

You've been using traits since the enums chapter.
Every time you wrote `#[derive(Debug, PartialEq)]` on an enum or struct, you were asking the compiler to write the `impl Debug for ...` and `impl PartialEq for ...` blocks for you.
That's all `derive` is: a macro that emits the obvious implementation so you don't have to type it out.
We'll revisit this in a moment.

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

## Static vs. Dynamic Dispatch: A Sneak Preview

Throughout the exercises, you'll encounter two ways to use traits, called static and dynamic dispatch.
Here's what each looks like:

### Static Dispatch

With generics, the compiler specializes `print_all` for the concrete types you use it with.
The calls need no trait-object lookup, but every element in one call must have the same concrete type.
Many specializations can increase code size and compilation time.

```rust
fn print_all<T: Display>(items: &[T]) { /* ... */ }
```

### Dynamic Dispatch

The alternative is to use the `dyn` keyword.
A trait-object reference lets one function call methods on different concrete types through a shared interface.
Those calls use indirection, though the optimizer can sometimes remove it.
This can reduce the number of specialized functions, but it does not guarantee a smaller binary or faster compilation.

The following slice can mix different concrete types that all implement `Display`:

```rust
fn print_all_dyn(items: &[&dyn Display]) { /* ... */ }
```

### When to Use Each

Use a generic bound when each call works with one concrete type and a trait object when you need to handle different concrete types through the same reference type.
Both approaches require implementations known to the program; `dyn` doesn't remove compile-time checking of the interface.

You'll practice them separately below.
First implement traits for individual types, then write a generic caller.
Next try inheriting and overriding default methods in separate editors.
Finally implement validation rules, borrow a mixed shelf, and collect results from different validators.

Each editor has one job and includes its own support code and tests.
Some ask you to write an entire implementation or function, so their first run intentionally reports missing code rather than reaching a `todo!()`.
If you work locally, run one file independently with `rustc --edition=2024 --test examples/16_traits/03_describable.rs -o /tmp/traits-test && /tmp/traits-test` (change the filename for another step).
A chapter-wide `cargo test` needs every missing definition filled in, even when a test-name filter selects only one step.
