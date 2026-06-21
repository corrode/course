# Memory and ownership

You've been using ownership for a while now without making a fuss about it.
You moved `String`s, borrowed slices, passed `&mut` references into functions, and handed values to `Vec`, `HashMap`, `Option`, `Result`, and your own structs.
This chapter steps back and names what you've been doing, because the pieces add up to the feature Rust is best known for: memory safety without a garbage collector.

## What the borrow checker buys you

Languages with manual memory management (C, C++) hand you the power to free memory yourself, and with it the power to free it twice, free it too early, or forget to free it at all.
Languages with a garbage collector take that power back and spend runtime and memory tracking what's still alive.
Rust takes a third path: the compiler proves, before the program runs, that every value is freed exactly once and that no reference outlives what it points at.

Three rules make that proof possible, and you've already met all of them:

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You can borrow a value without owning it, under the aliasing rule (many `&`, or one `&mut`, never both).

Rules 1 and 2 rule out double-frees and leaks: there's always exactly one owner to do the cleanup, and it happens automatically at the end of the scope.
Rule 3 rules out data races and use-after-free: you can't write through one reference while reading through another, and you can't hold a reference to something that's already been dropped.

The payoff is that "did I free this?" and "is this pointer still valid?" stop being questions you answer at 2am with a debugger.
The compiler answers them for you on every build.
