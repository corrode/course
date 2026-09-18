# Validating required variables

At startup, an app often needs to verify its database URL, port, or API key before doing any work.
Your helper takes a list of required keys and reports the first one that's missing.

`Iterator::find` is a good fit: scan the slice, return the first key that isn't in the map, and turn that into an `Err`.
If `find` returns `None`, every required key was present and the result is `Ok(())`.

## Useful from the standard library

- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter) on `required` yields `&&str` (a reference to each `&str` in the slice).
- [`Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) borrows each item for its predicate, so `|key|` receives `&&&str`, but the returned item is `Option<&&str>`.
  Use `!env.contains_key(**key)` to find a missing key: [`HashMap::contains_key`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key) accepts a `&str` lookup for a `String` key.
- [`Option::map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) collapses the result into a `Result` in one call: `find(...).map_or(Ok(()), |k| Err(k.to_string()))`.
- A plain `for k in required { if !env.contains_key(*k) { return Err(...) } }` loop reads just as well; pick whichever you like.
