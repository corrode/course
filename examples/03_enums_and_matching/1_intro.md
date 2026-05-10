# Enums and pattern matching

An `enum` is a type whose value is one of a fixed set of variants. Think
of it as a "this or that or that" type.

```rust
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}
```

You typically inspect an enum value with `match`. The compiler insists you
handle every variant, which is one of Rust's most loved features: when you
add a new variant later, every `match` that didn't account for it stops
compiling and tells you exactly where to update.

```rust
fn code(status: HttpStatus) -> u16 {
    match status {
        HttpStatus::Ok => 200,
        HttpStatus::NotFound => 404,
        HttpStatus::InternalServerError => 500,
    }
}
```

Each arm of a `match` is `pattern => expression`. Multiple patterns can
share an arm with `|`, and the catch-all is `_`:

```rust
match code {
    200 | 201 | 204 => "success",
    404 => "missing",
    _ => "something else",
}
```

`#[derive(Debug, PartialEq)]` on an enum gives you `{:?}` printing and
`==`/`!=` comparisons for free. You'll see those derives often.

## Useful from the standard library

- [`std::cmp::PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
  is the trait that lets you use `==` on a value. `#[derive(PartialEq)]`
  asks the compiler to write the implementation for you.
- [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
  enables the `{:?}` format specifier, the easiest way to print an enum
  while debugging.
- [`std::matches!`](https://doc.rust-lang.org/std/macro.matches.html)
  is a macro for "does this value match this pattern?" Returns `bool`.
  Compact when you only care about one variant.
- [The Rust Book on `match`](https://doc.rust-lang.org/book/ch06-02-match.html)
  has more patterns: ranges, guards, binding with `@`, and so on.
