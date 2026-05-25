# Trait objects: `dyn Trait`

The generic `print_descriptions<T: Describable>` from step 3 is fast
and zero-cost, but it has one limit: every element of a single call
must be the same concrete `T`. You can pass `&[Book]` or `&[Movie]`,
but not a slice that contains *both*.

That's because the compiler picks one `T` per call site and produces a
specialized copy of the function for it. The slice type `&[T]` has to
agree on a single element type, and two different structs are two
different types as far as the type system is concerned.

## Enter `dyn Trait`

When you want a single collection that holds *different* concrete
types as long as they all implement the same trait, you reach for a
**trait object**, spelled `dyn Trait`:

```rust
fn run_all(items: &[&dyn Validator], input: &str) {
    for v in items {
        let _ = v.check(input);
    }
}
```

Two things changed compared to the generic version:

1. **Element type.** `&dyn Validator` is a *fat pointer*: two words
   that together point at the value and at a vtable of function
   pointers (one per trait method). At each call to `.check(...)`,
   Rust looks the function up in the vtable. C++ folks: this is the
   same machinery as virtual methods, just opt-in per call site
   instead of per class.
2. **One function, not many.** `run_all` is compiled exactly once.
   The price is the indirection through the vtable; the payoff is
   heterogeneous collections.

## Static vs. dynamic dispatch, side by side

| | `fn f<T: Trait>(x: &T)` | `fn f(x: &dyn Trait)` |
| --- | --- | --- |
| Dispatch | static, decided at compile time | dynamic, vtable lookup at runtime |
| Code size | one copy per `T` you use | one copy total |
| Mixed collections | no | yes |
| Runtime cost | none | one indirect call per trait-method call |

Neither is "better." Use generics by default for performance and
flexibility, and reach for `dyn Trait` when you genuinely need
heterogeneous storage or want a smaller binary.

## What this step adds

You'll build a tiny composable validation library. The trait is
deliberately small:

```rust
trait Validator {
    /// `Ok(())` on success, `Err(message)` on failure.
    fn check(&self, input: &str) -> Result<(), String>;
}
```

Three implementors, each enforcing one rule:

- `MinLength { n }`: input must have at least `n` characters.
- `MustContain { needle }`: input must contain the given substring.
- `MustNotContain { forbidden }`: input must *not* contain the
  given substring.

The killer feature is that `MinLength`, `MustContain`, and
`MustNotContain` are three different types, but a single
`&[&dyn Validator]` slice can hold all of them at once. That's what
makes "pluggable rules" work: every rule is a different struct (and
might carry different configuration), but the call site only knows
"a list of things that can validate."

You'll see the same idea, scaled up, in Chapter 18's password
validator: a configurable set of checks running against one input.

## A word about `Box<dyn Trait>`

You'll see `Box<dyn Trait>` everywhere in real Rust code:

```rust
let rules: Vec<Box<dyn Validator>> = vec![
    Box::new(MinLength { n: 8 }),
    Box::new(MustContain { needle: "@".to_string() }),
];
```

The reason: `dyn Trait` has no statically known size (the three
implementors above can carry different fields, so they don't all
take up the same number of bytes), so the compiler won't let you
put bare `dyn Validator` values directly in a `Vec`. A `Box` is a
heap allocation with a fixed-size pointer that lives on the stack,
which sidesteps the size problem. That's exactly what Chapter 15
on smart pointers unpacks. For this step we sidestep it with
`&dyn Validator` references, which are also fixed-size and work
fine for borrowing.

## Useful from the standard library

- [The Rust Book on trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html).
- `str::contains` (with a `&str` argument) is all you need for the
  `MustContain` / `MustNotContain` checks.
- Inside `collect_errors`, a plain `for` loop pushing into a
  `Vec<String>` is the most direct form. The same chain with
  `.iter().filter_map(...)` works once you've met iterators in
  Chapter 16.
