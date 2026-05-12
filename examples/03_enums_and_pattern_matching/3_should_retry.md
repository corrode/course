# Matching one variant

Sometimes you only care about a single variant. You can still write a
full `match` with a `_` catch-all arm, or you can reach for the
[`matches!`](https://doc.rust-lang.org/std/macro.matches.html) macro
— both are idiomatic.

Here, only a server-side failure (`InternalServerError`) is worth
retrying. Client errors like `NotFound` or `BadRequest` mean the
request itself is wrong, so retrying won't help.
