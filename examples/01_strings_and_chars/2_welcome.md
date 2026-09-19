# A Welcome Message

Time to put `&str` and `String` together. Implement `format_welcome_message` so
it returns the string `"Welcome, {name}!"`.

The signature already tells you what to do:

```rust
fn format_welcome_message(name: &str) -> String
```

You borrow the `&str` to read the name and return a new `String` that the caller
owns.
