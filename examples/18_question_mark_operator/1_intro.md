# The `?` operator

Working with `Result` quickly becomes verbose if every call needs a match-then-return:

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

The `?` operator is shorthand for that pattern.
Slap it onto any `Result` expression: if it's `Ok`, the value is unwrapped and execution continues; if it's `Err`, the function returns the error immediately.

```rust
fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
```

`?` only works inside a function whose return type is also a `Result` (or `Option`).
It doesn't work in `fn main()` unless `main` itself returns a `Result`.

## Mixing error types

What if one call returns `ParseIntError` and another returns `io::Error`?
The compiler needs them to agree.
The two common solutions:

1. **`Box<dyn Error>`**: a trait-object error type that accepts almost any error.
   Quick and cheap, great for small programs and prototypes.

   ```rust
   fn run() -> Result<i32, Box<dyn std::error::Error>> {
       let text = std::fs::read_to_string("nums.txt")?;   // io::Error
       let n: i32 = text.trim().parse()?;                 // ParseIntError
       Ok(n * 2)
   }
   ```

   This is where `Box<dyn Error>` finally pays off.
   You met both halves of that signature in the previous two chapters: `Box<T>` is the heap-owning smart pointer from the smart pointers chapter, and `dyn Error` is the trait-object syntax from the traits chapter ("any type that implements the `Error` trait").
   Putting them together gives you "a heap-allocated, any-error-goes return type" that `?` can convert into from almost any concrete error.

2. **A custom error enum** with a variant per underlying error.
   More work, but the type system tells callers exactly what can go wrong.
   You'll see the [`thiserror`](https://docs.rs/thiserror) crate used for this in real projects.

`?` knows how to convert between error types if there's a `From` implementation.
`Box<dyn Error>` works because nearly every error type has `From<E> for Box<dyn Error>`.

## A note on these tests and the filesystem

The tests below write real files (`test.txt`, `numbers.txt`, …) into the current working directory before they run.
Cargo runs tests in parallel by default, so two tests writing to the same path can race each other and cause spurious failures.
If you see flaky `Err`s here, force the harness to run them one at a time:

```sh
cargo test -- --test-threads=1
```

Or give each test its own filename if you're feeling tidy.
In production code you'd reach for [`tempfile::NamedTempFile`](https://docs.rs/tempfile) so the OS hands you a guaranteed-unique path and cleans up after itself.


