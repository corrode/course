# `?` for the simple case

`?` is Rust's shortcut for "if this is `Err`, return it from the
current function; otherwise, unwrap the value and continue." It
compresses a lot of `match` boilerplate into one character.

Here both calls to `parse()` return the *same* error type
([`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)),
so `?` works directly without any conversion. Compare to writing this
out with a `match` on each `parse()` result. That's the boilerplate
`?` is replacing.

See: <https://doc.rust-lang.org/std/primitive.str.html#method.parse>
