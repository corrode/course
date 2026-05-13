# Option<T>: when a value might be missing

*I looked for a joke about null pointers in Rust, but there was `None`.*

Rust has no `null`. Instead, when a value might be absent, the type makes
that explicit using `Option<T>`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The compiler will not let you accidentally use a `None` as if it were a
real value. Every time you have an `Option`, you have to deal with both
cases. That's the whole point.

There are two main ways to unwrap an option. Pattern matching is the
fundamental tool:

```rust
match find_user(id) {
    Some(name) => println!("found {name}"),
    None => println!("no such user"),
}
```

But for common cases there are shorter combinators:

```rust
let port = settings.port.unwrap_or(8080);     // value or fallback
let upper = name.map(|s| s.to_uppercase());   // transform if Some
let len = maybe_str.map_or(0, |s| s.len());   // transform-or-default
```

### A note on `|x| ...` (closures)

Those `|s| s.to_uppercase()` and `|s| s.len()` bits are *closures*:
anonymous functions you can pass as arguments. The pipes hold the
parameters; everything after them is the body:

```rust
let add_one = |x| x + 1;
add_one(2); // 3
```

If the body needs multiple statements, wrap it in braces:

```rust
let greet = |name: &str| {
    let trimmed = name.trim();
    format!("hello, {trimmed}")
};
```

Closures show up properly in chapter 11. For this chapter, just read
`|s| s.len()` as "a tiny one-shot function that takes `s` and returns
`s.len()`."

A useful one to know: `if let` lets you handle just the `Some` case
without writing a full `match`:

```rust
if let Some(user) = find_user(id) {
    println!("welcome, {user}");
}
```

Many standard-library methods return `Option`. `.first()`, `.last()`,
`.next()` on iterators, `.get()` on slices and maps, `.find(...)` on
iterators. You'll meet `Option` everywhere.


