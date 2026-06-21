# Adding items

Now we modify the list in place.
Where the previous step only read from the `Vec`, this one needs to change it.
The `&mut Vec<String>` says "I need exclusive access for a moment", and that's what lets us push a new item onto the end.

## Useful from the standard library

- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  appends one item to the end of the vector.
  Requires `&mut self`, which is why the parameter here is `&mut Vec<String>`.
- [`str::to_string`](https://doc.rust-lang.org/std/primitive.str.html#method.to_string)
  (or [`String::from`](https://doc.rust-lang.org/std/string/struct.String.html#method.from))
  turns the borrowed `&str` parameter into the owned `String` the vector wants to hold.
