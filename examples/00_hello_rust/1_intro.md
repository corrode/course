# Welcome

Glad you're here. This is the first exercise. It's deliberately tiny so
you can see how the rest of the course works without any friction.

Each chapter follows the same shape: a short note like this one, a code
editor with a function to fill in, and tests that turn green when your
code is right. Hit **Run** (or press `Ctrl/Cmd + Enter`) to compile and
test in the browser.

If you'd rather work locally, the
[course repo](https://github.com/corrode/course) has the same files;
`cargo test --example NN_slug` runs a single chapter.

## A few habits that pay off

Rust rewards small steps. The compiler is strict, and that's the point:
it will tell you exactly what's wrong if you let it.

- **Compile early, compile often.** A function that returns `todo!()`
  compiles. Build outward from there, one change at a time.
- **Read the error messages slowly.** Rust's compiler is unusually
  helpful. The `help:` and `note:` lines often tell you exactly what to
  change.
- **Look things up.** The
  [`std` docs](https://doc.rust-lang.org/std/) are the friend you'll
  reach for most. Press `?` from any page to pop open the
  [Cheatsheet](/cheatsheet) for the syntax used in this course.
- **Use the [Playground](/playground) for side experiments.** A quick
  `println!` to see what `"café".len()` returns, a `match` to check
  syntax, anything you don't want to clutter the exercise with.

If a chapter has you stuck, every exercise has a **Stuck? Show a hint**
disclosure under the editor. The hints reveal one step at a time so you
get unblocked without giving away the whole answer.

That's the whole loop. Let's write some Rust.
