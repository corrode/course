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
