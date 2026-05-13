# `?` across multiple error types

Now the function combines two distinct error sources: parsing a
string into a number ([`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html))
and reading a file ([`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)).
With a fixed error type you'd need to convert each one by hand.
`Box<dyn std::error::Error>` is the laziest catch-all: any type that
implements the [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
trait can be boxed into it, and `?` does the conversion automatically
because both `ParseIntError` and `io::Error` implement `Error`.

In real code you'd typically define your own error enum (or reach for
a crate like `anyhow` or `thiserror`). `Box<dyn Error>` is the
standard-library escape hatch.

## Useful from the standard library

- [`std::fs::read_to_string`](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
  reads the file. Use `?` to propagate the I/O error.
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)
  walks the lines. Pair with [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim)
  to drop stray whitespace before parsing.
- For the per-line parse, an iterator pipeline with
  `.map(|l| l.parse::<i32>()).sum::<Result<i32, _>>()` collects the
  total in one call: `sum` on a `Result`-yielding iterator returns
  the first error or the total. The `?` after that reduces it back
  to `i32`.
- Returning `Box<dyn std::error::Error>` works because both error
  types implement the `Error` trait and `?` knows how to box them.
