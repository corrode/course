# Wrapping up word count

Three tiny functions, all cut from the same template: a counter variable, a `for` loop, and a return statement.
That's enough to build a real, useful tool, and it's the same shape you'll keep reaching for as the chapters get bigger.

## What we learned

- `text.split_whitespace()` walks the words in a string for you.
  It handles any kind of whitespace and skips empties without ceremony.
- `text.chars()` walks every character in a string, whitespace and all.
  It's the right tool for "how many characters?".
- "Track the running maximum" is the same shape every time: `let mut max = 0; for x in xs { if x > max { max = x; } }`.
- `word.chars().count()` measures string length in characters, which is usually what you want.
  `str::len` returns *bytes*, and the two differ the moment you hit a non-ASCII character.

## What comes next

You'll meet `split_whitespace` and `chars` again in **the iterators chapter**, where the three loops you just wrote collapse to:

```rust
fn word_count(text: &str)    -> usize { text.split_whitespace().count() }
fn char_count(text: &str)    -> usize { text.chars().count() }
fn longest_word(text: &str)  -> usize { text.split_whitespace().map(|w| w.chars().count()).max().unwrap_or(0) }
```

Then in **the word frequencies chapter** we'll go further and ask not just *how many* words a text contains, but *which* words appear and *how often* each one shows up.
