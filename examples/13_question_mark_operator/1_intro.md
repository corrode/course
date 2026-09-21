# The `?` Operator

"Handle all errors" is great advice! ...unless you end up with boilerplate like this: 

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

The `?` operator is shorthand for that pattern. Put it after a `Result`
expression to get the value from `Ok` and continue. If the result is `Err`, the
function returns the error immediately.

```rust
fn parse_two(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}
```

For these exercises, `?` on a `Result` needs a function that also returns a
`Result`. It also works on `Option` inside functions returning `Option`, where
`None` causes an early return. To use `?` on a `Result` in `main`, give `main` a
`Result` return type.


