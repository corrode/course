# Searching a collection

Produce an `Option` by searching a slice of user records.
The search returns a reference to a whole `(u32, String)` tuple, while the function must return only the username as `Option<&str>`.
Your job is to bridge those two types without cloning the `String`.

## Useful from the standard library

- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter)
  yields shared references to the slice's items, one at a time.
- [`Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
  takes a predicate closure and returns the first matching item as an `Option`.
- [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
  transforms the inner value when present.
  Here it pulls the username out of the tuple and converts it to `&str`.
- [`String::as_str`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_str)
  is the explicit "borrow this `String` as `&str`" call.
