# Iterators

*Rust iterators are the best thing since `&[bread]`.*

Iterators turn "loop over a collection and do something" into a pipeline.
They're lazy: nothing actually runs until you ask for a result.
The compiler usually fuses chained iterator calls into a single tight loop, so the abstraction is free at runtime.

Here's how iterators work in practice:

1. Get an iterator with `.iter()`, `.into_iter()`, `.iter_mut()`, or directly from `.chars()`, `.lines()`, etc.
2. Chain *adapters* like `.map(...)`, `.filter(...)`, `.take(...)`.
   These are lazy.
3. Finish with a *consumer* like `.collect()`, `.sum()`, `.count()`, `.any(...)`, or a `for` loop.

```rust
let names = vec!["alice", "ADMIN", "bob"];

let active: Vec<String> = names
    .iter()                              // &&str
    .filter(|n| n.starts_with('a'))      // keep some
    .map(|n| n.to_lowercase())           // transform
    .collect();                          // back to Vec<String>
// active == ["alice"]
```

The three "iter" methods differ in what they yield:

- `.iter()` yields `&T` (immutable references).
  Use when reading.
- `.iter_mut()` yields `&mut T`.
  Use when modifying in place.
- `.into_iter()` yields `T` (consumes the collection).
  Use when you don't need the original anymore.

Some adapters change the item type.
After `.map(|n| n.to_lowercase())`, the items are `String`s, not `&&str`s.
The compiler infers types through the chain, so trust it: write the chain, then add a type annotation on the binding if needed.

`.collect()` is interesting: it can produce many different collections.
Tell it which one with a type annotation: `Vec<_>`, `HashMap<_, _>`, `String`.
The `_` lets the compiler fill in the inner types.

## Coming back to word count

Remember the three little functions from the word count chapter's exercise break?
Each one was a counter, a `for` loop, and a return.
With iterators, the whole trio shrinks to:

```rust
fn word_count(text: &str)   -> usize { text.split_whitespace().count() }
fn char_count(text: &str)   -> usize { text.chars().count() }
fn longest_word(text: &str) -> usize {
    text.split_whitespace().map(|w| w.chars().count()).max().unwrap_or(0)
}
```

The loops, the `mut` counters, the running-maximum bookkeeping — all gone.
That's the payoff for spending a chapter on iterators: every "walk a collection and reduce it to one number, or to one new collection" problem gets shorter and harder to get wrong.


