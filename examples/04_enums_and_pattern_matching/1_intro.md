# Enums and pattern matching

*“Dad, why is my sister’s name Rose?”  
“Because your mother loves roses.”  
“Thanks, dad!”  
“No problem, Rust enums.”*

Dad is right, enums are the best.
If you know the crippled form of enums in other languages (*cough* C), I'm so sorry for you.
In Rust, they are a pure delight to work with.

But first things first: an `enum` is a type whose value is one of a fixed set of variants. Think
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


