# Default methods

A trait method can ship with a default body.
Implementors get that method for free, but any one of them can override it if the default doesn't fit.

```rust
trait Greet {
    fn name(&self) -> &str;

    // Default body. Implementors get this for free.
    fn hello(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}
```

A type that says `impl Greet for X { fn name(&self) -> &str { "world" } }` automatically has `.hello()` available, returning `"Hello, world!"`, without writing it out.
Provide a `fn hello` in the impl block and yours wins instead.

This is how `Iterator` gets away with offering dozens of methods (`map`, `filter`, `sum`, `count`, ...) while only requiring you to implement one: `fn next(&mut self) -> Option<Self::Item>`.
Every other method is a default body written in terms of `next`.
Haskellers will recognise the pattern from type class default methods; Java added the same feature as "default methods on interfaces" in Java 8.

## A logger with shared behavior

The `Logger` trait has one required method, `log`, which formats a single line.
Its `warn` and `error` methods provide default bodies built on top of `log`.

```rust
trait Logger {
    fn log(&self, msg: &str) -> String;

    fn warn(&self, msg: &str) -> String {
        self.log(&format!("[WARN] {msg}"))
    }

    fn error(&self, msg: &str) -> String {
        self.log(&format!("[ERROR] {msg}"))
    }
}
```

The shape should look familiar from logging APIs in other languages.
Because `warn` and `error` live on the *trait*, each new implementor gets both without writing them again.

You'll implement that contrast with two types:

1. `PlainLogger` returns the message untouched.
   It uses both defaults as written, so all you have to write is `log`.
2. `TaggedLogger { tag: String }` prepends a tag (e.g. `"auth: ..."`).
   It uses the default `warn`, but *overrides* `error` to swap the `[ERROR]` prefix for a louder `[CRITICAL]` prefix.

That asymmetry lets each implementor keep the defaults that fit and replace only the behavior that differs.

## Useful from the standard library

- The `format!` macro is the workhorse here.
  Default `warn` builds `"[WARN] {msg}"` and hands it back to `self.log`, so whatever decoration `log` does (the tag, in `TaggedLogger`'s case) wraps the warning prefix.
- Default methods are written *inside* the `trait` block, with a body instead of a trailing semicolon.
  A method ending in a semicolon remains required, while a method with a body can be inherited or overridden.
