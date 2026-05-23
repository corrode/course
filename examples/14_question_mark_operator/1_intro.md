# The `?` operator

Working with `Result` quickly becomes verbose if every call needs a
match-then-return:

```rust
fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = match a.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let y = match b.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(x + y)
}
```

The `?` operator is shorthand for that pattern. Slap it onto any `Result`
expression: if it's `Ok`, the value is unwrapped and execution continues;
if it's `Err`, the function returns the error immediately.

```rust
fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
```

`?` only works inside a function whose return type is also a `Result` (or
`Option`). It doesn't work in `fn main()` unless `main` itself returns a
`Result`.

## Mixing error types

What if one call returns `ParseIntError` and another returns `io::Error`?
The compiler needs them to agree. The two common solutions:

1. **`Box<dyn Error>`**: a trait-object error type that accepts almost
   any error. Quick and cheap, great for small programs and prototypes.

   ```rust
   fn run() -> Result<i32, Box<dyn std::error::Error>> {
       let text = std::fs::read_to_string("nums.txt")?;   // io::Error
       let n: i32 = text.trim().parse()?;                 // ParseIntError
       Ok(n * 2)
   }
   ```

   This is the first time we've reached for a *smart pointer* and a
   *trait object*, so a quick translation of the syntax:

   - `Box<T>` is the simplest smart pointer in the standard library:
     it owns a `T` that lives on the heap, and frees it when the
     `Box` is dropped. If you've used C++, it's `std::unique_ptr<T>`
     with the same single-owner rule Rust uses everywhere.
   - `dyn Error` means "some value whose concrete type we don't
     know at compile time, but which implements the `Error` trait."
     The `dyn` keyword marks it as a *trait object*, essentially a
     pair of pointers: one to the value, one to its method table.
     This is Rust's version of runtime polymorphism, the same idea
     as a Java interface reference or a C++ virtual base pointer.
   - We need the `Box<...>` wrapper around `dyn Error` because the
     real value can be any size, and Rust insists that the things
     it stores directly have a size known at compile time. Boxing
     it puts the unknown-sized value behind a known-size pointer.

   Traits, trait objects, and smart pointers each get more attention
   in later material; for now, read `Box<dyn Error>` as "a heap-
   allocated, any-error-goes return type."

2. **A custom error enum** with a variant per underlying error. More
   work, but the type system tells callers exactly what can go wrong.
   You'll see the [`thiserror`](https://docs.rs/thiserror) crate used for
   this in real projects.

`?` knows how to convert between error types if there's a `From`
implementation. `Box<dyn Error>` works because nearly every error type
has `From<E> for Box<dyn Error>`.

## A note on these tests and the filesystem

The tests below write real files (`test.txt`, `numbers.txt`, …) into the
current working directory before they run. Cargo runs tests in parallel
by default, so two tests writing to the same path can race each other
and cause spurious failures. If you see flaky `Err`s here, force the
harness to run them one at a time:

```sh
cargo test -- --test-threads=1
```

Or give each test its own filename if you're feeling tidy. In production
code you'd reach for [`tempfile::NamedTempFile`](https://docs.rs/tempfile)
so the OS hands you a guaranteed-unique path and cleans up after itself.


