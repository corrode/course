# Validating required variables

Most apps need *some* configuration to be present at startup: a database URL, a port, an API key.
This last helper takes a list of required keys and reports the first one that's missing.

`Iterator::find` is a good fit: scan the slice, return the first key that isn't in the map, and turn that into an `Err`.
If `find` returns `None`, every required key was present and the result is `Ok(())`.

## Useful from the standard library

- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter) on `required` yields `&&str` (a reference to each `&str` in the slice).
  The closure parameter `|key|` is therefore `&&&str`, which auto-derefs through method calls.
- [`Iterator::find`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find) takes a predicate and returns the first matching item as `Option<&&&str>`.
  Combine with [`HashMap::contains_key`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.contains_key) to look for keys not in the map.
- [`Option::map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or) collapses the result into a `Result` in one call: `find(...).map_or(Ok(()), |k| Err(k.to_string()))`.
- A plain `for k in required { if !env.contains_key(*k) { return Err(...) } }` loop reads just as well; pick whichever you like.
