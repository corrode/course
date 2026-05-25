# Word Frequencies

Time to extend our running word-count example. Back in chapter 5 you
built `word_count`, `char_count`, and `longest_word` with simple `for`
loops. Chapter 16 then showed how iterators collapse those into
one-liners. This chapter goes one level deeper: instead of asking *how
many* words a text has, we'll ask *which* words appear and *how often
each one* shows up.

There's no big new concept here either — it puts iterators, hashmaps,
and `Option` to work together. Along the way you'll meet two new
iterator tricks (`max_by_key` and `HashMap::into_iter`); the rest is
just applying what's already in your toolbox.

A few patterns you'll likely use:

**Splitting text into words.** Both `split_whitespace` and `split` return
iterators of `&str`. The first handles any kind of whitespace and skips
empties, which is usually what you want for natural text:

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

**Finding the maximum by some property.** `max_by_key` is the right tool
for "give me the entry with the largest count":

```rust
let top = counts.iter().max_by_key(|(_, count)| *count);
// top: Option<(&String, &usize)>
```

**Filtering and collecting.** Same iterator chain you saw in the
previous chapter:

```rust
let frequent: Vec<String> = counts
    .iter()
    .filter(|(_, &n)| n >= min)
    .map(|(word, _)| word.clone())
    .collect();
```

**Computing an average.** Sum the lengths, divide by the count, watch out
for the integer-division trap:

```rust
let total_chars: usize = words.iter().map(|w| w.len()).sum();
let avg = total_chars as f64 / words.len() as f64;
```


