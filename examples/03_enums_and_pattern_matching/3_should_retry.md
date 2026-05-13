# Matching one variant

Sometimes you only care about a single variant. You can still write a
full `match` with a `_` catch-all arm, or you can reach for the
[`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro.
Both are idiomatic.

Here, only a server-side failure (`InternalServerError`) is worth
retrying. Client errors like `NotFound` or `BadRequest` mean the
request itself is wrong, so retrying won't help.

## Useful from the standard library

- [`std::matches!`](https://doc.rust-lang.org/std/macro.matches.html)
  expands to a `match` that returns `true` for the given pattern and
  `false` otherwise. Reads naturally as
  `matches!(status, HttpStatus::InternalServerError)`.
- The `==` operator works on enums that derive `PartialEq`, so
  `status == HttpStatus::InternalServerError` is equally fine.
  Pick whichever reads better at the call site.
