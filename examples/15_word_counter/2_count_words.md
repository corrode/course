# Counting words

The foundation for everything else in this chapter: take a string of
text and produce a `HashMap<String, usize>` that maps each word to
how many times it appears. Words are separated by whitespace and the
count should be case-insensitive — `"Hello"` and `"hello"` are the
same word.

The classic recipe is: split on whitespace, lowercase each piece,
then walk the resulting iterator and bump a counter in the map. The
`entry` API on `HashMap` is the idiomatic way to do that last step:
`*map.entry(key).or_insert(0) += 1`.
