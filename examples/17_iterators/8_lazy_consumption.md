# Predict, then run: how far does it go?

The tests below already contain the pipelines.
Before running them, replace each `todo!()` with your prediction; leave the pipelines unchanged.

For the first test, predict how many times the closure runs when nothing consumes the iterator.
For the second, predict the first result, the collected batch, the next result, and the inputs visited by `map`.
Write down the order of the printed messages too.
Then run the tests and compare the trace with your predictions.
Locally, use `cargo test --example 17_iterators _8_lazy_consumption:: -- --nocapture` to see output from passing tests.

- [`next`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next) asks for one item and returns an `Option`.
- [`by_ref`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.by_ref) lets a consumer borrow an iterator so you can use it again afterward.
- [`take`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take) limits how many items its consumer can request.

Does asking for one filtered result visit just one input?
After collecting the batch, does the next call start over or resume?
Try changing `take(1)` to `take(2)`, predict all the results again, and rerun.
