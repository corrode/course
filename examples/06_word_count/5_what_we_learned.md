# Wrapping up word count

You wrote three functions using a counter or running maximum, a `for` loop, and a return value.

## What we learned

- `text.split_whitespace()` walks the words in a string for you.
  It handles any kind of whitespace and skips empty pieces.
- `text.chars()` walks every Unicode scalar value in a string, whitespace and all.
- To track the running maximum, use `let mut max = 0; for x in xs { if x > max { max = x; } }`.
- `word.chars().count()` counts `char` values, not necessarily visible characters.
  `str::len` returns *bytes*, and the two differ the moment you hit a non-ASCII character.

## What comes next

Iterator methods let you write the same three functions without explicit loops:

```rust
fn word_count(text: &str)    -> usize { text.split_whitespace().count() }
fn char_count(text: &str)    -> usize { text.chars().count() }
fn longest_word(text: &str)  -> usize { text.split_whitespace().map(|w| w.chars().count()).max().unwrap_or(0) }
```

The word-frequencies example extends this from counting all words to recording which words appear and how often.

Want more practice? Try the optional [word count challenge](06_word_count_challenge).
It does not count toward course completion.
