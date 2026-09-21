# Iterators

*What do managers and Rust iterators have in common? They can both look
very productive while doing absolutely nothing.*

Rust iterators are lazy: they don't do any work until you ask for a result.
That's great, because it means you can chain together many operations without
creating intermediate collections.

You can filter, transform, and combine values in a single pipeline. And once you
get the hang of it, you'll find that iterators are often more readable than a
`for` loop. That's because you don't have to manage indexes or check bounds
yourself; it's all part of Rust's ergonomic iterator machinery, one of my
favorite parts of the language.

Here's how iterators work in practice:

1. Get an iterator with `.iter()`, `.into_iter()`, `.iter_mut()`, or directly
   from `.chars()`, `.lines()`, etc.
2. Chain *adapters* like `.map(...)`, `.filter(...)`, `.take(...)`. These are
   lazy.
3. Finish with a *consumer* like `.collect()`, `.sum()`, `.count()`,
   `.any(...)`, or a `for` loop.

```rust
let words = vec!["hello", "café", "rust"];

let lengths: Vec<usize> = words
    .iter()
    .map(|word| word.chars().count())
    .collect();
assert_eq!(lengths, vec![5, 4, 4]);
```

The three "iter" methods differ in what they yield:

- `.iter()` yields `&T` (immutable references). Use when reading.
- `.iter_mut()` yields `&mut T`. Use when modifying in place.
- `.into_iter()` yields `T` (consumes the collection). Use when you don't need
  the original anymore.

Some adapters change the item type. Here, `iter` yields `&&str`, but `map`
produces character counts of type `usize`. The compiler infers types through
the chain.
You can write the chain first, then add a type annotation on the binding if the
compiler needs one.

`.collect()` can produce many different collections. Tell it which one with a
type annotation: `Vec<_>`, `HashMap<_, _>`, `String`. The `_` lets the compiler
fill in the inner types.

## Coming Back to Word Count

Remember the three little functions from the
[word count chapter](06_exercise_break_word_count)'s exercise break? Each
one was a counter, a `for` loop, and a return. With iterators, the whole trio
shrinks to:

```rust
fn word_count(text: &str)   -> usize { text.split_whitespace().count() }
fn char_count(text: &str)   -> usize { text.chars().count() }
fn longest_word(text: &str) -> usize {
    text.split_whitespace().map(|w| w.chars().count()).max().unwrap_or(0)
}
```

You no longer have to maintain the `mut` counters or keep track of the maximum
yourself.


