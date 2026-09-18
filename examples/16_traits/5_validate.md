# Trait Objects: `dyn Trait`

The generic `print_descriptions<T: Describable>` you wrote earlier avoids runtime dispatch, but every element in a single call must have the same concrete type `T`.
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

## Static vs. Dynamic Dispatch, Side by Side

| | `fn f<T: Trait>(x: &T)` | `fn f(x: &dyn Trait)` |
| --- | --- | --- |
| Dispatch | static, decided at compile time | dynamic, vtable lookup at runtime |
| Code size | one copy per `T` you use | one copy total |
| Mixed collections | no | yes |
| Runtime cost | none | one indirect call per trait-method call |

I'd start with generics for performance and flexibility.
Reach for `dyn Trait` when you need to store different concrete types together or want a smaller binary.

## A Validation Example

You'll use trait objects to build a small validation library.
Each validator needs one method:

```rust
trait Validator {
    /// `Ok(())` on success, `Err(message)` on failure.
    fn check(&self, input: &str) -> Result<(), String>;
}
```

Each struct has one rule; `MinLength` is already implemented for you:

- `MinLength { n }`: input must have at least `n` characters.
- `MustContain { needle }`: input must contain the given substring.
- `MustNotContain { forbidden }`: input must *not* contain the given substring.

`MinLength`, `MustContain`, and `MustNotContain` are different types, but one `&[&dyn Validator]` slice can hold all three.
Each rule can carry its own configuration while the call site only needs a list of values that can validate.

If you take the optional password validator later, you'll use the same idea for a configurable set of checks.

## A Word about `Box<dyn Trait>`

You'll also see `Box<dyn Trait>` in Rust code:

```rust
let rules: Vec<Box<dyn Validator>> = vec![
    Box::new(MinLength { n: 8 }),
    Box::new(MustContain { needle: "@".to_string() }),
];
```

`dyn Trait` has no statically known size.
The three implementors above can carry different fields, so they don't all take up the same number of bytes.
That's why the compiler won't let you put bare `dyn Validator` values directly in a `Vec`.
A `Box` owns a heap allocation through a fixed-size pointer, which sidesteps the size problem.
The pointer itself can live on the stack or inside another allocation, such as a `Vec` buffer.
`Box<dyn Trait>` is the owning form of this fixed-size handle.
`&dyn Validator` borrows through a fixed-size handle instead of taking ownership.

## Useful from the Standard Library

- [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains) (with a `&str` argument) is all you need for the `MustContain` / `MustNotContain` checks.
- Inside `collect_errors`, a plain `for` loop pushing into a `Vec<String>` is the most direct form.
  An `.iter().filter_map(...)` chain expresses the same loop with iterator adapters.

## Stop or Keep Going?

For `"a b"`, predict the messages from all three rules before running the test.
Why must `collect_errors` continue after a failed check, unlike the `?` operator?
Would changing its return type to `Result<Vec<String>, String>` and adding `?`
preserve that behavior? Explain which failures the caller would lose.
Then try the mixed shelf task after the hints.
