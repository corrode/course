# Word Frequencies

Back in the [word count chapter](/exercise/exercise_break_word_count) you built
`word_count`, `char_count`, and `longest_word` with simple `for` loops. Then you
used iterators to collapse those loops into one-liners. Now we'll expand on that
to write more complex text analysis tools.

Our implementation combines iterators, hash maps, and `Option`, as well as
`max_by_key` and `HashMap::into_iter`, which are two new iterator tools we'll
get to know in this chapter. 

## Splitting Text into Words

Both `split_whitespace` and `split` return iterators of `&str`. The first
handles any kind of whitespace and skips empties, which is usually what you want
for natural text:

```rust
for word in "hello  world\nrust".split_whitespace() {
    println!("{word}"); // hello, world, rust
}
```

## Choosing by a Key

`max_by_key` compares items using a value you derive from each one. Here we
choose a filename by its length rather than its alphabetical order:

```rust
let files = ["a.rs", "settings.rs", "main.rs"];
let longest = files.iter().max_by_key(|name| name.len());
assert_eq!(longest, Some(&"settings.rs"));
```

## Computing an Average

Add the lengths, convert the totals to `f64`, and only then divide so integer
truncation can't discard the fraction:

```rust
let total_chars: usize = words.iter().map(|w| w.chars().count()).sum();
let avg = total_chars as f64 / words.len() as f64;
```


