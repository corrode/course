# Wrapping up traits

You implemented an stdlib trait (`Display`), defined your own
(`Describable`), wrote a generic function with a trait bound, gave
a trait a default method that one type inherited and another
overrode (the `Logger` step), and finally swapped a generic for a
trait object so a single slice could hold a mix of validation rules.

## What we learned

- A **trait** is a named collection of method signatures. Any type
  can opt in with `impl TraitName for TypeName { ... }`. Same idea
  as Java/C# interfaces, Haskell type classes, Swift protocols, or
  C++ abstract classes with pure virtual methods.
- **`#[derive(...)]`** is sugar: the compiler writes the obvious
  `impl Trait for Type` block for you. `Debug`, `PartialEq`, `Eq`,
  `Clone`, `Copy`, `Default`, `Hash`, and `Ord` are the everyday
  derivable ones. `Display` is *not* derivable because there's no
  one-size-fits-all human-readable format.
- **`impl Display`** is the `toString` / `__str__` of Rust. Implement
  `fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result` with a single
  `write!(...)` call.
- **Trait bounds** on a generic say "I accept any `T` that implements
  this trait." `fn f<T: Trait>(x: &T)` is the basic form, `T: A + B`
  combines bounds, and `where` clauses let you push long bounds out
  of the signature.
- **Default methods** in the trait body give every implementor a
  baseline behavior. Override per type when you need to.
- **Generics** dispatch statically: one specialized copy of the
  function per concrete `T`. Fast, but one call can only see one
  concrete type. Great default.
- **Trait objects (`dyn Trait`)** dispatch dynamically through a
  vtable. Slower per call but lets one slice or `Vec` hold values of
  multiple concrete types at once. Reach for them when you need
  heterogeneity.
- `Box<dyn Trait>` solves the "trait objects have no known size"
  problem so they can live in owning containers like `Vec`. That's the
  headline of the optional **smart pointers** bonus chapter (`Box`,
  `Rc`, `RefCell`) — dip into it whenever you want to go deeper.
- Many stdlib traits you already use (`Iterator`, `From`, `Into`,
  `PartialEq`, ...) are just regular traits. Nothing magic. You can
  define your own or implement the standard ones for your own types.
