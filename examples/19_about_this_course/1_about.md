# About this course

You finished the course (or you're skipping ahead, which is fine too).
A bit of background, in case you're curious.

## Why this course exists

There's no shortage of Rust resources. The
[official book](https://doc.rust-lang.org/book/) is excellent, and you
should read it. So is
[Rust by Example](https://doc.rust-lang.org/rust-by-example/), and
plenty of others.

What was missing, for me, was a path you could open in a browser and
just *do*. Not a book to read on the couch, not a lecture to watch on
2x speed: a sequence of small, real problems with tests that turn green.
Something where the code leaves your fingers and lands in the machine
right away.

These exercises started as a folder on my laptop, the kind of thing I'd
pull out whenever someone asked "where do I start with Rust?" Over the
years it grew into a structured path through the language. This site is
that path, with an editor attached.

## Why Rust

There are a lot of programming languages. Here's why I keep coming back
to this one.

**It goes everywhere.** Embedded firmware, command-line tools, web
backends, browser frontends via WebAssembly, kernel modules. One
language, one toolchain, one mental model. That breadth is rare and
quietly empowering.

**The pieces fit.** Every type and method in Rust feels deliberately
placed. Once a few core ideas click (ownership, traits, the type
system), the rest of the language stops surprising you. Code starts to
*snap* into place.

**The compiler is on your side.** Rust's strictness gets a bad rap, but
in practice it means the program you finally get past the borrow
checker tends to work, including the cases you didn't think to test.
The error messages are some of the best in the industry.

**It teaches you about computers.** Even if you never write Rust at
work, the time spent here will change how you think about memory,
concurrency, and what your other languages are doing under the hood.

## How to keep going

The course covers the core mechanics. Real fluency comes from using
those mechanics on problems that matter to you. A few suggestions:

- **Build something tiny.** A CLI that does one thing for you. A
  scraper. A toy interpreter. The smaller and more personal, the
  better. Finishing it teaches more than reading another tutorial.
- **Read other people's Rust.** Pick a small crate you find useful and
  read its source. The Rust ecosystem leans toward clean, idiomatic
  code; you'll absorb a lot just by skimming.
- **Use it for the next thing you'd otherwise do in another language.**
  The first project will be slow. The second will be noticeably less
  slow. By the fifth, Rust feels like home.
- **Stay curious.** Rust is a deep language; nobody knows all of it.
  When something surprises you, that's the language inviting you to dig
  in.

## A note from corrode

This course is open source on
[github.com/corrode/course](https://github.com/corrode/course).
Issues and pull requests welcome. If you'd like Rust training, code
review, or consulting for your team, see
[corrode.dev](https://corrode.dev).

Thanks for spending time here. Now go write some Rust.
