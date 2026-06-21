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

## A note on these tests and the filesystem

The test below writes a real file (`test.txt`) into the current working directory before it runs.
Cargo runs tests in parallel by default, so two tests writing to the same path can race each other and cause spurious failures.
If you see flaky `Err`s here, force the harness to run them one at a time:

```sh
cargo test -- --test-threads=1
```

Or give each test its own filename if you're feeling tidy.
In production code you'd reach for [`tempfile::NamedTempFile`](https://docs.rs/tempfile) so the OS hands you a guaranteed-unique path and cleans up after itself.


