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

## `#[derive(...)]`: free implementations

You'll see this line on many types in Rust:

```rust
#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}
```

The `#[...]` syntax is an *attribute*: extra instructions for the
compiler attached to the item below. `derive` is the most common one.
It says "please write the boilerplate for these capabilities for me."
Each name inside the parentheses is a *trait* (Rust's name for a
shared interface, similar to a Java interface or a Haskell type
class; traits get their own chapter later).

The two we use right away:

- **`Debug`** lets you print the value with the `{:?}` formatter, so
  `println!("{status:?}")` prints `NotFound` instead of refusing to
  compile. Useful in `dbg!`, `assert_eq!` failure messages, and quick
  log lines.
- **`PartialEq`** generates `==` and `!=`. Without it, comparing two
  `HttpStatus` values is a compile error; with it, `status ==
  HttpStatus::Ok` just works, and `assert_eq!` in tests can compare
  whole enum values.

Derive works on enums and structs whose fields all implement the
same traits. The compiler writes the obvious implementation. For
`PartialEq` on an enum, that means "two values are equal if they're
the same variant with equal payloads." You can always write the
implementation by hand instead when you need different behaviour.


