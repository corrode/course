# `?` with a different error type

Same operator, different error type. File I/O returns
[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html);
the function signature has to declare it as the error type so `?` is
happy passing it through.

Notice that `?` doesn't care which error type is involved as long as
the function it's used in returns *the same* error type (or one
convertible from it via `From`, which is the next step).

See: <https://doc.rust-lang.org/std/fs/fn.read_to_string.html>
