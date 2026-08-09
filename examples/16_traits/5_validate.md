# Trait objects: `dyn Trait`

The generic `print_descriptions<T: Describable>` from step 3 is fast and zero-cost, but it has one limit: every element of a single call must be the same concrete `T`.
You can pass `&[Book]` or `&[Movie]`, but not a slice that contains *both*.

That's because the compiler picks one `T` per call site and produces a specialized copy of the function for it.
The slice type `&[T]` has to agree on a single element type, and two different structs are two different types as far as the type system is concerned.

## Using `dyn Trait`

When you want a single collection that holds *different* concrete types as long as they all implement the same trait, you reach for a **trait object**, spelled `dyn Trait`:

```rust
fn run_all(items: &[&dyn Validator], input: &str) {
    for v in items {
        let _ = v.check(input);
    }
}
```

`&dyn Validator` is a *fat pointer*: two words that point to the value and to a vtable of function pointers, one for each trait method.
At each call to `.check(...)`, Rust looks up the function in that vtable.
C++ folks will recognize the same machinery as virtual methods, but here you opt into it at the call site instead of on the class.
`run_all` is also compiled exactly once rather than once per concrete type.
That trades one vtable lookup per call for the ability to mix concrete types in the slice.

## Static vs. dynamic dispatch, side by side

| | `fn f<T: Trait>(x: &T)` | `fn f(x: &dyn Trait)` |
| --- | --- | --- |
| Dispatch | static, decided at compile time | dynamic, vtable lookup at runtime |
| Code size | one copy per `T` you use | one copy total |
| Mixed collections | no | yes |
| Runtime cost | none | one indirect call per trait-method call |

Neither is "better."
Use generics by default for performance and flexibility, and reach for `dyn Trait` when you genuinely need heterogeneous storage or want a smaller binary.

## A validation example

You'll use trait objects to build a small validation library.
Each validator needs one method:

```rust
trait Validator {
    /// `Ok(())` on success, `Err(message)` on failure.
    fn check(&self, input: &str) -> Result<(), String>;
}
```

You'll give three structs one rule each:

- `MinLength { n }`: input must have at least `n` characters.
- `MustContain { needle }`: input must contain the given substring.
- `MustNotContain { forbidden }`: input must *not* contain the given substring.

`MinLength`, `MustContain`, and `MustNotContain` are different types, but one `&[&dyn Validator]` slice can hold all three.
Each rule can carry its own configuration while the call site only needs a list of values that can validate.

If you take the optional password validator later, you'll use the same idea for a configurable set of checks.

## A word about `Box<dyn Trait>`

You'll also see `Box<dyn Trait>` in Rust code:

```rust
let rules: Vec<Box<dyn Validator>> = vec![
    Box::new(MinLength { n: 8 }),
    Box::new(MustContain { needle: "@".to_string() }),
];
```

The reason: `dyn Trait` has no statically known size (the three implementors above can carry different fields, so they don't all take up the same number of bytes), so the compiler won't let you put bare `dyn Validator` values directly in a `Vec`.
A `Box` is a heap allocation with a fixed-size pointer that lives on the stack, which sidesteps the size problem.
We'll unpack that ownership pattern in the optional smart pointers material.
For now, `&dyn Validator` references give us the same fixed-size handle without taking ownership.

## Useful from the standard library

- [The Rust Book on trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).
- `str::contains` (with a `&str` argument) is all you need for the `MustContain` / `MustNotContain` checks.
- Inside `collect_errors`, a plain `for` loop pushing into a `Vec<String>` is the most direct form.
  Once we get to iterators, the same loop can become an `.iter().filter_map(...)` chain.
