# Result<T, E>: when an operation might fail

*Failure is not an `Option<T>`, but a `Result<T, E>`.*

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

The snippet above sneaks in three pieces of syntax that don't have their
own chapter, so it's worth pausing on each:

### `&'static str`

This is a `&str` whose lifetime is `'static`: a fancy way of saying
"this string lives for the entire duration of the program." String
literals like `"port must be greater than 0"` are baked into the
binary, so they qualify. For now, treat `&'static str` as the right
type to use for hard-coded error messages. Lifetimes in general get a
more careful treatment in later chapters.

### Turbofish: `parse::<u16>()`

`.parse()` doesn't know which type you want to parse into, so you tell
it with the funny-looking `::<T>` syntax. It's just a way to spell out a
generic type argument at the call site:

```rust
let n = "42".parse::<u16>().unwrap();
// equivalent if the type can be inferred from context:
let n: u16 = "42".parse().unwrap();
```

You'll see this anywhere a function returns `T` and the type isn't clear
from the surrounding code.

### Match guards: `Ok(n) if n > 0 => ...`

The `if n > 0` clause on a match arm is called a *guard*. The arm only
fires when both the pattern matches *and* the guard is true. Without it
you'd need a nested `if` inside the arm body, which reads worse.

```rust
match n {
    x if x < 0 => "negative",
    0          => "zero",
    _          => "positive",
}
```

### Back to `Result`

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


