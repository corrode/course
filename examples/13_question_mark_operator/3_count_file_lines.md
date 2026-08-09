# `?` with a different error type

Here you use the same operator with a different error type.
File I/O returns [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html), and this function declares that same error type so `?` can pass failures back unchanged.

`?` doesn't care which concrete error type is involved.
It only needs the surrounding function to return the same error type, or one it can convert into with `From`.

## Useful from the standard library

- [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) reads the whole file into a `String`.
  The returned `Result<String, io::Error>` matches this function's error type.
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) iterates over the file's lines without keeping the trailing newlines.
- [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) consumes the iterator and returns how many lines there were.
- Once you have the file contents, `lines()` and `count()` can remain in the same expression.
