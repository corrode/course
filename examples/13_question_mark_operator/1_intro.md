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

2. **A custom error enum** with a variant per underlying error. More
   work, but the type system tells callers exactly what can go wrong.
   You'll see the [`thiserror`](https://docs.rs/thiserror) crate used for
   this in real projects.

`?` knows how to convert between error types if there's a `From`
implementation. `Box<dyn Error>` works because nearly every error type
has `From<E> for Box<dyn Error>`.

## Useful from the standard library

- [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
  reads a whole file into a `String`. Returns `io::Error`.
- [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
  is the canonical error type for I/O.
- [`std::num::ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)
  is what `str::parse::<i32>()` returns on failure.
- [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
  is the trait that all error types implement. `Box<dyn Error>` is a
  trait object based on it.
- [The Rust Book on `?`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
  has the formal explanation.
