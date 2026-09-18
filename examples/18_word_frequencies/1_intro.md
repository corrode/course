# Word frequencies

Time to extend our running word-count example.
Back in the word count chapter you built `word_count`, `char_count`, and `longest_word` with simple `for` loops.
Then you used iterators to collapse those loops into one-liners.
This time you'll keep track of which words appear and how often each one occurs.

The implementation combines iterators, hash maps, and `Option`.
`max_by_key` and `HashMap::into_iter` are the only new iterator tools needed here.

## Splitting text into words

Both `split_whitespace` and `split` return iterators of `&str`.
The first handles any kind of whitespace and skips empties, which is usually what you want for natural text:

```rust
for word in "hello  world\nrust".split_whitespace() {
    println!("{word}"); // hello, world, rust
}
```

## Counting into a HashMap

Use `entry(...).or_insert(0)` to get each word's counter, starting at zero if you haven't seen the word before:

```rust
let mut counts: HashMap<String, usize> = HashMap::new();
for word in text.split_whitespace() {
    *counts.entry(word.to_lowercase()).or_insert(0) += 1;
}
```

## Finding the largest count

`max_by_key` lets you ask for the entry with the largest count:

```rust
let top = counts.iter().max_by_key(|(_, count)| *count);
// top: Option<(&String, &usize)>
```

## Computing an average

Add the lengths, convert the totals to `f64`, and only then divide so integer truncation can't discard the fraction:

```rust
let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();
let avg = total_chars as f64 / words.len() as f64;
```


