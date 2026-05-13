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
