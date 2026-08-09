# Word frequencies

Time to extend our running word-count example.
Back in the word count chapter you built `word_count`, `char_count`, and `longest_word` with simple `for` loops.
Then you used iterators to collapse those loops into one-liners.
Word frequencies answer two related questions: which words appear, and how often does each one occur.

The implementation combines iterators, hash maps, and `Option`.
`max_by_key` and `HashMap::into_iter` are the only new iterator tools needed here.

**Splitting text into words.** Both `split_whitespace` and `split` return iterators of `&str`.
The first handles any kind of whitespace and skips empties, which is usually what you want for natural text:

```rust
for word in "hello  world\nrust".split_whitespace() {
    println!("{word}"); // hello, world, rust
}
```

**Counting things into a HashMap.** Reach for `entry(...).or_insert(0)`:

```rust
let mut counts: HashMap<String, usize> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word.to_lowercase()).or_insert(0) += 1;
}
```

**Finding the maximum by some property.** `max_by_key` is the right tool for "give me the entry with the largest count":

```rust
let top = counts.iter().max_by_key(|(_, count)| *count);
// top: Option<(&String, &usize)>
```

**Computing an average.** Add the lengths, convert the totals to `f64`, and only then divide so integer truncation can't discard the fraction:

```rust
let total_chars: usize = words.iter().map(|w| w.len()).sum();
let avg = total_chars as f64 / words.len() as f64;
```


