# Building a List from Borrowed Slices

This time the input and output hold different string types: each input is a
borrowed `&str`, while the output must own its `String`s. That means every item
needs to become an owned `String` before it can live in the result.

## Useful from the Standard Library

- [`Vec::new`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
  creates an empty vector.
- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  appends one item.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  copies the text into an owned `String`, as in `add_item`.
