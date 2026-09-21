# The `?` Operator

"Handle all errors" is great advice! ...unless you end up with boilerplate like this:

```rust
fn is_even(text: &str) -> Result<bool, std::num::ParseIntError> {
    let number = match text.parse::<i32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(number % 2 == 0)
}
```

The `?` operator is shorthand for that pattern. Put it after a `Result`
expression to get the value from `Ok` and continue. If the result is `Err`, the
function returns the error immediately.

```rust
fn is_even(text: &str) -> Result<bool, std::num::ParseIntError> {
    let number = text.parse::<i32>()?;
    Ok(number % 2 == 0)
}
```

For these exercises, `?` on a `Result` needs a function that also returns a
`Result`. It also works on `Option` inside functions returning `Option`, where
`None` causes an early return. To use `?` on a `Result` in `main`, give `main` a
`Result` return type.


