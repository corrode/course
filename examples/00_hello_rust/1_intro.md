# Welcome

Glad you're here. This is your first exercise. It's deliberately tiny,
just enough to see how the rest of the course works.

Each chapter follows the same shape: a short note like this one, a code
editor with a function to fill in, and tests that turn green when your
code is right.

If a chapter has you stuck, every exercise has a **Stuck? Show a hint**
disclosure under the editor. The hints reveal one step at a time so you
get unblocked without giving away the whole answer.

<details>
<summary><strong>A few habits that pay off</strong></summary>

Rust rewards small steps. The compiler is strict, and that's the point:
it will tell you exactly what's wrong if you let it.

- **Compile early, compile often.** A function that returns `todo!()`
  compiles. Build outward from there, one change at a time.
- **Read the error messages slowly.** Rust's compiler is unusually
  helpful. The `help:` and `note:` lines often tell you exactly what to
  change. If a message mentions an error code (e.g. `E0382`), running
  `rustc --explain E0382` locally gives the long version.
- **Look things up.** The
  [`std` docs](https://doc.rust-lang.org/std/) are the friend you'll
  reach for most. Press `?` from any page to pop open the
  [Cheatsheet](/cheatsheet) for the syntax used in this course.
- **Use the [Playground](/playground) for side experiments.** A quick
  `println!` to see what `"café".len()` returns, a `match` to check
  syntax, anything you don't want to clutter the exercise with.

### Getting unstuck

Everyone gets stuck. These are the moves that tend to work.

- **Start small.** Don't try to write the whole function in one go.
  Get *something* compiling first, even if it just returns a dummy
  value. `todo!()` compiles. `return Vec::new()` compiles. Use those
  as scaffolding while you figure out the real logic.
- **Break the problem down.** What are the inputs? What's the output?
  What's one small step that gets you closer to the output? Implement
  that step, run the tests, repeat.
- **Read the standard library docs.** When you're not sure which
  method to call on a `String`, `Vec`, `Option`, or `HashMap`, the
  answer is almost always one click away at
  [doc.rust-lang.org/std](https://doc.rust-lang.org/std/). Note the
  type, open its docs page, skim the method list. Names are usually
  self-explanatory.
- **Take a short break.** A walk away from the keyboard solves a
  startling number of "impossible" bugs. Explaining the problem out
  loud (to a colleague, or to a rubber crab) often surfaces the
  answer on its own.

</details>

That's the whole loop. Let's write some Rust.
