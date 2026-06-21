# Methods that borrow `&self`

A method taking `&self` reads the struct's fields without modifying or consuming it.
The most common kind.
Inside the method, `self` behaves like any other reference, so you can read fields freely and the caller keeps ownership.

`display_name` formats two fields into a new `String`.
Use `format!` rather than building the string by hand.
It's the idiomatic tool for this and reads exactly like the format you want.

## Useful from the standard library

- [`format!`](https://doc.rust-lang.org/std/macro.format.html) builds a new `String` from a template and arguments.
  Same syntax as `println!`, but returns the string instead of printing it.
- Field access uses dot notation: `self.name`, `self.email`.
  Inside a `format!` template you can interpolate them inline: `format!("{} ({})", self.name, self.email)`.
