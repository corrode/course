# Transforming the inside

Same idea as before, but now the fallback isn't the value itself. You
need to call `.len()` on the inner string first. A `match` makes both
branches explicit; iterator-style methods on `Option` are tidier once
you spot them.

See: <https://doc.rust-lang.org/std/keyword.match.html>
