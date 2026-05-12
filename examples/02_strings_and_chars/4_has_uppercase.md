# Iterating over characters

Strings aren't directly indexable in Rust (because UTF-8 characters
have varying widths), but you can iterate over their `char`s. A
plain `for c in text.chars()` loop will work, and so will the
iterator combinators like `any` or `find` — which usually express
"is there at least one ..." checks more directly.
