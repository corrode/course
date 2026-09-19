# A Beginner's Guide to Rust 

![Screenshot of the course page](static/assets/screenshot.jpg)

A hands-on Rust course for working developers. You write small programs, the
compiler gives you feedback, and after a couple of dozen exercises you start to
get the hang of it.

This is an official course repository by [corrode](https://corrode.dev), a Rust
consultancy that helps teams adopt Rust in production.

## How It Works

The course runs primarily through the website, which walks you through each
chapter, shows the exercises in your browser, and tracks your progress. That's
the recommended way to take it.

You can work through it on your own or with a team. With a team, you can submit
your solutions and compare approaches with each other. Either way, take it at
your own pace.

If you'd rather stay in your editor, this is also just a regular Rust project.
Clone it, open it in your IDE, and work through the exercises under `examples/`.

Each chapter is a numbered directory under `examples/`. Work through its `.md`
prose and `.rs` exercises in filename order. Replace the `todo!()` bodies in the
exercise files and run the tests until they pass. The chapter's generated
`main.rs` only connects the steps for Cargo, so don't edit it directly.

If you get stuck, there's a sample solution for every exercise under
`solutions/`. It mirrors the chapter and filename layout under `examples/`.
These are the same solutions the website reveals in its hints.

The workflow is roughly:

```bash
git clone https://github.com/corrode/course.git
cd course
# Edit examples/00_integers/3_add_health.rs, then test the whole chapter:
cargo test --example 00_integers
# Or run just that step's tests:
cargo test --example 00_integers _3_add_health::
```

## About corrode

[corrode](https://corrode.dev) helps teams adopt Rust in production: training,
code reviews, and architecture work. If you'd like a workshop or a review of
your codebase, [get in touch](https://corrode.dev/services).
