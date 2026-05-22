# Your first function: a welcome message

Time to put `&str` and `String` together. Implement `format_welcome_message`
so it returns the string `"Welcome, {name}!"`.

The signature already tells the story:

```rust
fn format_welcome_message(name: &str) -> String
```

You're handed a borrowed `&str` to read from, and you produce a fresh,
owned `String` to hand back. The intro mentions `println!`, but
`println!` *prints*; it returns `()`. The macro that builds a `String`
for you to return is
[`format!`](https://doc.rust-lang.org/std/macro.format.html), which
uses the same `{name}` placeholder syntax.
