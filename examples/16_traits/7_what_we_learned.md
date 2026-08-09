# Wrapping up traits

You implemented the standard library's `Display` trait and defined a `Describable` trait of your own.
You used `Describable` as a generic bound, then shared behavior through default methods.
Finally, you switched from a generic to a trait object so one slice could hold several kinds of validation rule.

## What we learned

- A **trait** is a named collection of method signatures.
  Any type can opt in with `impl TraitName for TypeName { ... }`.
  Same idea as Java/C# interfaces, Haskell type classes, Swift protocols, or C++ abstract classes with pure virtual methods.
- **`#[derive(...)]`** is sugar: the compiler writes the obvious `impl Trait for Type` block for you.
  `Debug`, `PartialEq`, `Eq`, `Clone`, `Copy`, `Default`, `Hash`, and `Ord` are the everyday derivable ones.
  `Display` is *not* derivable because there's no one-size-fits-all human-readable format.
- **`impl Display`** is the `toString` / `__str__` of Rust.
  Implement `fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result` with a single `write!(...)` call.
- **Trait bounds** on a generic say "I accept any `T` that implements this trait."
  `fn f<T: Trait>(x: &T)` is the basic form, `T: A + B` combines bounds, and `where` clauses let you push long bounds out of the signature.
- **Default methods** in the trait body give every implementor a baseline behavior.
  Override per type when you need to.
- **Generics** dispatch statically, so the compiler creates one specialized copy of the function per concrete `T`.
  Use them when each call only needs one concrete type.
- **Trait objects (`dyn Trait`)** dispatch dynamically through a vtable.
  Use them when one slice or `Vec` needs to hold several concrete types at once.
- `Box<dyn Trait>` solves the "trait objects have no known size" problem so they can live in owning containers like `Vec`.
  We'll unpack that pattern alongside `Box`, `Rc`, and `RefCell` in the optional **smart pointers** material.
- Many standard library traits you already use (`Iterator`, `From`, `Into`, `PartialEq`, ...) follow the same rules.
  You can define traits of your own or implement standard traits for your own types.
