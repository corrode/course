# Turning feedback into suggestions

The validator (next step) produces a `PasswordReport` with a list of
short complaints in `feedback`, like `"too short"` or `"missing digit"`.
This step turns those complaints into actionable, human-readable
suggestions.

Decide on your own format. A reasonable approach is to scan each
feedback string for keywords ("short", "uppercase", "digit", ...) and
emit a matching suggestion ("Add at least 4 more characters", "Mix in
an uppercase letter like A-Z", ...).

The shared types are duplicated here so this step compiles on its own.

## Useful from the standard library

- [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
  takes either a `&str` or a `char` and answers yes/no. Perfect for
  keyword sniffing inside each feedback line.
- [`<[T]>::iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter)
  on `report.feedback` walks the messages without consuming the
  report. The closure inside an `if`/`else` chain can decide what
  suggestion (if any) to emit.
- [`Vec::push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
  appends each suggestion. A `Vec<String>` is a fine return value.
- [`format!`](https://doc.rust-lang.org/std/macro.format.html) lets
  you splice the input password into a suggestion when that helps
  the user understand what to fix.
