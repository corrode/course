# Searching a collection

The trickiest of the four: produce an `Option` by searching. The
iterator chapter is still ahead, but `slice::iter()` plus a search
combinator already gets you most of the way; the matched tuple still
needs to be reduced down to just the username.

Type walk-through (this is the puzzle):

- `users.iter()`                    yields `&(u32, String)`
- `.find(|(uid, _)| *uid == id)`    yields `Option<&(u32, String)>`
- `.map(|(_, name)| name.as_str())` yields `Option<&str>` (the return type)

`name.as_str()` turns the `&String` we destructured out of the tuple
into the `&str` the signature wants.

## Useful from the standard library

- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter)
  yields shared references to the slice's items, one at a time.
- [`Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find)
  takes a predicate closure and returns the first matching item as
  an `Option`.
- [`Option::map`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map)
  transforms the inner value when present. Here it pulls the
  username out of the tuple and converts it to `&str`.
- [`String::as_str`](https://doc.rust-lang.org/std/string/struct.String.html#method.as_str)
  is the explicit "borrow this `String` as `&str`" call.
