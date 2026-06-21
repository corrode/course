# Building a list from borrowed slices

The trickiest of the three: each input is a `&str`, but the output is a `Vec<String>`.
Each borrowed slice has to become an owned `String` somewhere along the way.
The `String::from` / `.to_string()` / `.to_owned()` family all do this conversion.

## Useful from the standard library

- [`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
  creates an empty vector you can push into.
  The [`vec!`](https://doc.rust-lang.org/std/macro.vec.html) macro is more common when you already know the contents.
- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  appends one item.
  Combine with a `for` loop over `items` to fill the result.
- [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from),
  [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string),
  and [`str::to_owned`](https://doc.rust-lang.org/std/primitive.str.html#method.to_owned)
  all turn a `&str` into a fresh `String`.
  Pick whichever reads best.
