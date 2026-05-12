# Filter, then own the result

Same shape as the previous step, but the input is a `&[&str]` (a
borrowed slice of borrowed strings), so the iterator yields `&&str`.
We sidestep that double-reference by returning owned `String`s; the
lesson here is iterators, not lifetimes.

To go from `&&str` to `String`, reach for [`str::to_string`]. Chain
it after your `filter` with a `map`, then `collect` into a `Vec`.
