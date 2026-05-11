# A first pass: comma-splitting

Before tackling the messy realities of CSV (quotes, escapes, embedded
commas), let's handle the trivial case: a line that's nothing but
plain values separated by commas, possibly with surrounding
whitespace. This is what most "I'll just split on commas" CSV
parsers do — and it's also why so many of them break.

Use [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split)
and [`str::trim`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).
Collect into a `Vec<String>`.
