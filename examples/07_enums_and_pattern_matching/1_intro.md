# Enums and Pattern Matching

*“Dad, why is my sister’s name Rose?”  
“Because your mother loves roses.”  
“Thanks, dad!”  
“No problem, Rust enums.”*

Dad is right, enums are the best. If you know the crippled form of enums in
other languages (*cough* C), I'm so sorry for you. In Rust, they are a pure
delight to work with.

An `enum` is a type whose value is one of a fixed set of variants. Think of it
as a "this or that or that" type.

```rust
enum Edit {
    Append(String),
    Clear,
}
```

A variant can carry data: `Edit::Append(String::from("!"))` owns the text to
append, while `Edit::Clear` needs no extra information.

You typically inspect an enum value with `match`. This is what I like about
`match`: the compiler checks that you've handled every variant. When you add a
new variant later, it points you to each `match` that no longer covers every
case.

```rust
fn apply_edit(text: &mut String, edit: Edit) {
    match edit {
        Edit::Append(suffix) => text.push_str(&suffix),
        Edit::Clear => text.clear(),
    }
}
```

Each arm of a `match` is `pattern => expression`. The `suffix` pattern binds
the string carried by `Append`. Here the arms modify text; a `match` can also
produce a value. Multiple patterns can share an arm with `|`, and the
catch-all is `_`:

```rust
let attempts = 2;
let advice = match attempts {
    0 => "not tried yet",
    1 | 2 => "try again",
    _ => "check the connection",
};
```

## `#[derive(...)]`: Free Implementations

You'll see this line on many types in Rust:

```rust
#[derive(Debug, PartialEq)]
enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
}
```

The `#[...]` syntax is an *attribute*: extra instructions for the compiler
attached to the item below. `derive` is the most common one. It says "please
write the boilerplate for these capabilities for me." Each name inside the
parentheses is a *trait*, Rust's name for a shared interface, similar to a Java
interface or a Haskell type class. Traits are covered in more detail later.

For now, you need two:

- `Debug` lets you print the value with the `{:?}` formatter, so
  `println!("{status:?}")` prints `NotFound` instead of refusing to compile. You
  can also use it in `dbg!`, `assert_eq!` failure messages, and quick log lines.
- `PartialEq` generates `==` and `!=`. Without it, comparing two `HttpStatus`
  values is a compile error; with it, `status == HttpStatus::Ok` just works, and
  `assert_eq!` in tests can compare whole enum values.

Derive works on enums and structs whose fields all implement the same traits.
For `PartialEq` on an enum, the generated implementation considers two values
equal when they have the same variant and equal payloads. You can always write
the implementation by hand instead when you need different behaviour.


