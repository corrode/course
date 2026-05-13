# Experiments: get the errors on purpose

Passing the previous tests is the easy part of this chapter.
Ownership only really clicks once you've seen the canonical errors
with your own eyes, so the messages feel familiar later (chapter 11,
chapter 13, …) instead of like a brick wall.

Each test below is paired with a commented-out line. Uncomment one
at a time, run the tests, read the error carefully, then comment it
out again before moving on. The errors *are* the lesson here. The
tests themselves don't assert anything interesting.

The three errors you'll trigger correspond to the three rules of
the borrow checker:

1. You can't use a value after you've moved it.
2. You can't have two mutable references to the same value at once.
3. You can't have a mutable reference while a shared reference is
   still in use.

Re-read each compiler message until you can explain in one sentence
*why* the compiler is complaining. That's the muscle this chapter is
building.

## Useful from the standard library

- [`Clone::clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone)
  makes an explicit deep copy when you genuinely need two owners. A
  good "escape hatch" once you've understood why a borrow won't
  compile, but not the first thing to reach for.
- The compiler errors themselves are the documentation here. Each
  one is a paragraph you'd otherwise have to read in a book.
