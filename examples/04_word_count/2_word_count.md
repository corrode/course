# Counting words

Your first function: given a string of text, return how many words
it contains. Words are anything separated by whitespace, so
`"hello world"` has two words and `"   "` has zero.

The recipe is the simplest possible: keep a counter, walk the text
with `for ... in text.split_whitespace()`, bump the counter on each
iteration, return it at the end.

## Useful from the standard library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
  walks through every whitespace-separated piece of a string. It
  handles tabs, newlines, and runs of consecutive spaces without
  any extra work on your part.
