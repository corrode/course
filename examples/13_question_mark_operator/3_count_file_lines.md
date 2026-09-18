# `?` with a Different Error Type

Here you use the same operator with a different error type.
File I/O returns [`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html), and this function declares that same error type so `?` can pass failures back unchanged.

`?` doesn't care which concrete error type is involved.
It needs the surrounding function to return the same error type, or one it can convert into with `From`.
When `?` fails to compile, I would check those two error types first.
Working out the conversion can take a few tries; this exercise avoids that by using `std::io::Error` throughout.

## Useful from the Standard Library

- [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html) reads the whole file into a `String`.
  The returned `Result<String, io::Error>` matches this function's error type.
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines) iterates over the file's lines without keeping the trailing newlines.
- [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) consumes the iterator and returns how many lines there were.
- Once you have the file contents, `lines()` and `count()` can remain in the same expression.

## The Test and the Filesystem

This test creates `test.txt` in the current working directory and removes it afterward.
It also expects `missing.txt` not to exist.
Avoid running multiple copies of this test in the same directory: they can overwrite or remove each other's file.
For tests in one process that share a path, `cargo test -- --test-threads=1` prevents them from running at the same time.
Separate test processes still need separate paths.

In production tests, [`tempfile::NamedTempFile`](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html) creates a uniquely named file and removes it when dropped.
