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

See: <https://doc.rust-lang.org/std/error/trait.Error.html>
