# Mapping Variants to Values

Write a `match` that turns each `HttpStatus` variant into the numeric code it
represents. If you forget one, the compiler points to the incomplete `match`
before the program can run.

After the tests pass, imagine adding `TooManyRequests` to the enum without
changing your `match`. Will it compile? Predict the result, then try it and
remove the extra variant. Would a `_` catch-all hide the missing mapping?

## Useful Resources

- [The Rust Book on `match`](https://doc.rust-lang.org/book/ch06-02-match.html)
  explains matching enum variants and handling every case.
