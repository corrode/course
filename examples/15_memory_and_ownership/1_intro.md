# Memory and ownership

You've already used ownership in several forms.
You moved `String`s, borrowed slices, passed `&mut` references into functions, and handed values to `Vec`, `HashMap`, `Option`, `Result`, and your own structs.
Now we can connect those operations and see how Rust manages memory safely without a garbage collector.

## What the borrow checker checks

Languages with manual memory management (C, C++) hand you the power to free memory yourself, and with it the power to free it twice, free it too early, or forget to free it at all.
Languages with a garbage collector take that power back and spend runtime and memory tracking what's still alive.
Rust takes a third path: before the program runs, the compiler checks where owned values are dropped and whether references outlive what they point at.

Three rules summarize the model you've already been using:

1. Every value has exactly one owner.
2. When the owner goes out of scope, the value is dropped.
3. You can borrow a value without owning it, under the aliasing rule (many `&`, or one `&mut`, never both).

Rules 1 and 2 give each value one place where cleanup normally happens, which rules out double-frees in safe Rust.
The aliasing part of rule 3 rules out data races because you can't write through one reference while reading through another.
Lifetime checks supply the other half: a reference can't remain usable after its owner has been dropped.

For an owned `String`, dropping the value frees its buffer automatically.
If a reference would still be used after that drop, safe Rust rejects the code at compile time.
