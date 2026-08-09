# Building a list from borrowed slices

This time the input and output hold different string types: each input is a borrowed `&str`, while the output must own its `String`s.
That means every item needs to become an owned `String` before it can live in the result.

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
