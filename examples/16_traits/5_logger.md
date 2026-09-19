# Default Methods

A trait method can ship with a default body.
Implementors get that method for free, but any one of them can override it if the default doesn't fit.

This is how `Iterator` gets away with offering dozens of methods (`map`, `filter`, `sum`, `count`, ...) while requiring only one method: `next`.
An iterator also declares its item type, which you'll meet in the next chapter.
Haskellers will recognise the pattern from type class default methods; Java added the same feature as "default methods on interfaces" in Java 8.

## A Logger with Shared Behavior

The `Logger` trait has one required method, `log`, which formats a single line.
Its `warn` and `error` methods provide default bodies built on top of `log`.
Because they live on the *trait*, each new implementor gets both without writing them again.

Implement `Logger` for `PlainLogger` so that `log` returns the message untouched as a `String`.
Write the whole impl block, but only implement the required method.
Leave `warn` and `error` to the trait: `warn("slow query")` should return `"[WARN] slow query"`, and `error("disk full")` should return `"[ERROR] disk full"`.
An empty message still gets the severity prefix and its trailing space.

In the next exercise, you'll keep one default and replace the other.

## Useful from the Standard Library

- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string) creates an owned `String` from a string slice.
