# `?` with a different error type

Same operator, different error type. File I/O returns
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html);
the function signature has to declare it as the error type so `?` is
happy passing it through.

Notice that `?` doesn't care which error type is involved as long as
the function it's used in returns *the same* error type (or one
convertible from it via `From`, which is the next step).

## Useful from the standard library

- [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
  reads the whole file into a `String`. Returns
  `Result<String, io::Error>`, which is exactly what `?` wants.
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
  iterates over the file's lines without keeping the trailing
  newlines.
- [`Iterator::count`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count)
  consumes the iterator and returns how many lines there were.
- The full body fits on one line:
  `Ok(std::fs::read_to_string(filename)?.lines().count())`.
