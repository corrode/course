# Getting Unstuck

Everyone gets stuck. The Rust compiler is strict, and when you're new to the
language a single line of code can produce an error message that looks like a
small essay. This page collects the strategies that tend to work when an
exercise (or any other piece of Rust code) is fighting back.

These are the same techniques we use ourselves on real projects. They're not
tricks; they're habits. The more you practice them, the less often you'll
need them.

## Start small

Don't try to write the whole function in one go. Get *something* compiling
first, even if it just returns a dummy value. Then change one thing at a
time and re-check.

A function body of `todo!()` compiles. A `return Vec::new();` compiles.
A `return 0;` compiles. Use these as scaffolding while you figure out the
real logic.

## Break down the problem

If a function feels too big, split it into smaller pieces in your head (or
on paper) before you write any code:

1. What are the inputs?
2. What's the output?
3. What's one small step that gets you closer to the output?

Then implement that one step, run the tests, and repeat.

## Compile often

Rust's compiler is one of the best teachers you'll ever have, but only if
you talk to it frequently. Save and run `cargo check` (or your editor's
equivalent) every time you change something non-trivial. Catching one
mistake at a time is much easier than catching ten at once.

`cargo check` is faster than `cargo build` because it skips code generation,
so use it for the tight feedback loop and only run the tests when you think
the code is ready.

## Read the error message

Rust's error messages are unusually good. They almost always:

- Point at the exact line and column.
- Explain what the compiler expected and what it got.
- Often suggest a fix (`help: ...`).

When an error looks intimidating, read it once at full speed, then read it
again slowly. Look for the `help:` and `note:` lines. They're usually the
answer.

If a message mentions a concept you don't know yet (lifetimes, traits,
borrowing rules), copy the error code (e.g. `E0382`) and run
`rustc --explain E0382` for a longer explanation.

## Read the standard library docs

When you're not sure which method to call on a `String`, `Vec`, `Option`, or
`HashMap`, the answer is almost always one click away at
[doc.rust-lang.org/std](https://doc.rust-lang.org/stable/std/).

A useful workflow:

1. Type the value into your editor.
2. Note its type.
3. Open the docs page for that type.
4. Skim the method list. The names are usually self-explanatory.

The standard library is small enough that you'll get familiar with it
surprisingly quickly.

## When all else fails

- Take a short break. A walk away from the keyboard solves a startling
  number of "impossible" bugs.
- Explain the problem out loud, to a colleague or to a rubber crab. The act
  of putting it into words often surfaces the answer.
- Ask for help. Don't sit stuck for an hour when five minutes with another
  person would unblock you.
