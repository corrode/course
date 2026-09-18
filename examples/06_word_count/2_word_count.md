# Counting Words

Start with the smallest piece of the library: given some text, return how many words it contains.
For this exercise, words are anything separated by whitespace, so `"hello world"` has two and `"   "` has none.

Use a `for` loop rather than an iterator counting method.

## Useful from the Standard Library

- [`str::split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) walks through every whitespace-separated piece of a string.
  It handles tabs, newlines, and runs of consecutive spaces without any extra work on your part.
