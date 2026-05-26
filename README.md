# A Beginner's Guide to Rust 

![Screenshot of the course page](/static/assets/screenshot.jpg)

A hands-on Rust course for working developers.
You write small programs, the compiler gives you feedback, and after a couple of
dozen exercises you start to get the hang of it. 

This is an official course repository by [corrode](https://corrode.dev), a Rust
consultancy that helps teams adopt Rust in production.

## How it works

The course runs primarily through the website, which walks you through
each chapter, shows the exercises in your browser, and tracks your
progress. That's the recommended way to take it.

You can start alone or with a team. If you're taking it with a team, you can share your
progress with each other by submitting your solutions and seeing other people's solutions.
The course is designed to be self-paced, so you can work through it at your own
speed, so no rush!

If you'd rather stay in your editor, remember that this is also just a regular Rust project!
You can clone it, open it in any IDE, and work through the exercises under `examples/`.

Each chapter is a directory under `examples/`. Open the files, read the
`//!` comment at the top, replace the `todo!()` bodies, and run the
tests until they pass. Then move on to the next chapter.

The workflow is roughly:

```bash
git clone https://github.com/corrode/course.git
cd course
# edit away, then...
cargo test --example 00_integers
```

## About corrode

[corrode](https://corrode.dev) helps teams adopt Rust in production:
training, code reviews, and architecture work. If you'd like a workshop
or a review of your codebase, [get in touch](https://corrode.dev/services).
