# Result<T, E>: when an operation might fail

`Result` is the sibling of `Option`. Where `Option` says "maybe a value,
maybe nothing", `Result` says "maybe a value, maybe an error":

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

This is how Rust handles fallible operations. There are no exceptions.
A function that can fail says so in its signature, and the caller has to
deal with both branches.

```rust
fn parse_port(input: &str) -> Result<u16, &'static str> {
    match input.parse::<u16>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err("port must be greater than 0"),
        Err(_) => Err("not a valid number"),
    }
}
```

The `&'static str` you see for the error type is the simplest possible
error: a borrowed string literal. Real applications usually define their
own error enums, but `&'static str` is fine while you're learning.

Patterns to handle a `Result`:

```rust
match safe_divide(10.0, 0.0) {
    Ok(n) => println!("got {n}"),
    Err(e) => println!("oops: {e}"),
}

let n = safe_divide(10.0, 2.0).unwrap_or(0.0);

if let Ok(n) = safe_divide(10.0, 2.0) {
    println!("got {n}");
}
```

`Result` has many of the same combinators as `Option`: `.map`, `.map_or`,
`.and_then`, `.unwrap_or`. Once you're comfortable with this chapter, the
`?` operator (chapter 13) will let you chain fallible operations without
the boilerplate.

## Useful from the standard library

- [`Result::is_ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok)
  and [`Result::is_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err)
  for quick boolean checks (handy in tests).
- [`Result::unwrap_or`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or)
  returns the value or a fallback, ignoring the error.
- [`Result::map`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map)
  transforms the `Ok` value while leaving `Err` alone.
- [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)
  transforms the error type. Useful when bubbling errors up across layers.
- [`Result::ok`](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok)
  converts `Result<T, E>` to `Option<T>`, throwing away the error info.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
  is the canonical "may fail to parse" function. Returns `Result`.
