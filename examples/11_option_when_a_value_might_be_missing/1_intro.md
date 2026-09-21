# Option<T>: When a Value Might Be Missing

*Here's my favorite joke about `null`:*

Anyway, Rust has no `null`. Instead, when a value might be absent, the type
makes that explicit using `Option<T>`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The compiler will not let you accidentally use a `None` as if it were a real
value. To extract the inner value, you need to decide what to do if it is
missing.

There are two main ways to unwrap an option. You can spell out both cases with
pattern matching:

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
let enabled = flag.map_or(false, |s| s == "on"); // transform-or-default
```

### A Note on `|x| ...` (Closures)

Those `|s| s.to_uppercase()` and `|s| s == "on"` bits are *closures*: anonymous
functions you can pass as arguments. The pipes hold the parameters; everything
after them is the body:

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

We'll use more closures in the [iterators chapter](/exercise/iterators). For
this chapter, just read `|s| s == "on"` as "a small function that takes `s` and
checks whether it's on."

When you only need to handle `Some`, you can use `if let` instead of a full
`match`:

```rust
if let Some(user) = find_user(id) {
    println!("welcome, {user}");
}
```

Many standard-library methods return `Option`: `.first()` and `.last()` on
slices, `.next()` and `.find(...)` on iterators, and `.get()` on slices and
maps.

