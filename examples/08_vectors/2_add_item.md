# Adding items

A `Vec` isn't frozen once you build it.
This step changes the list in place: it pushes a new item onto the end.
The `&mut Vec<String>` says "I need exclusive access for a moment", and that exclusive borrow is what lets us push.

## Useful from the standard library

- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  appends one item to the end of the vector.
  Requires `&mut self`, which is why the parameter here is `&mut Vec<String>`.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  (or [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from))
  turns the borrowed `&str` parameter into the owned `String` the vector wants to hold.
