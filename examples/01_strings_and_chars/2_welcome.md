# A welcome message

Time to put `&str` and `String` together.
Implement `format_welcome_message` so it returns the string `"Welcome, {name}!"`.

The signature already tells you what to do: 

```rust
fn format_welcome_message(name: &str) -> String
```

This means: you're handed a borrowed `&str` to read from, and you produce a fresh, owned `String` to hand back.
