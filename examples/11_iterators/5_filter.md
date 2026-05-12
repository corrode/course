# Keeping elements with `filter`

Same idea as `map`, but instead of transforming each element you keep
some and drop others. Watch out for one borrowing gotcha: `.iter()`
yields `&T`, but `filter` gives its closure another reference on top,
so the closure sees `&&T`.

That's why you'll often see `**c == ...` or `s.starts_with(...)` (which
auto-derefs) instead of plain `c == ...`. Don't be alarmed when the
compiler complains about a missing `&`, see
[the cheatsheet](/cheatsheet) entry on iterators.
