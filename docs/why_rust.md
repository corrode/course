---
title: Why Rust
status: draft
audience: blog
---

# Why Rust

There are a lot of programming languages. Here's why I keep coming back
to this one.

## It goes everywhere

Embedded firmware, command-line tools, web backends, browser frontends
via WebAssembly, kernel modules. One language, one toolchain, one
mental model. That breadth is rare and quietly empowering. The skills
transfer; the muscle memory transfers; even the libraries often
transfer.

## The pieces fit

Every type and method in Rust feels deliberately placed. Once a few
core ideas click (ownership, traits, the type system), the rest of the
language stops surprising you. Code starts to *snap* into place. There
are very few "well, that's just how it is" corners.

## The compiler is on your side

Rust's strictness gets a bad rap, but in practice it means the program
you finally get past the borrow checker tends to work, including the
cases you didn't think to test. The error messages are some of the
best in the industry: they point at the line, explain what the
compiler expected, and often suggest the fix.

It feels slow at first. Then it stops feeling slow, and you notice
how often other languages would have let a bug through.

## It teaches you about computers

Even if you never write Rust at work, time spent here will change how
you think about memory, concurrency, and what your other languages
are doing under the hood. Rust forces you to make decisions other
languages quietly make for you. That's tiring at first and clarifying
later.

## The community

The Rust community is unusually welcoming and unusually high-signal.
The standard library docs are written by people who care. So is most
of the ecosystem. You can learn a lot just by reading other people's
crates.

---

*This started as a section of the corrode Rust course appendix. Pulled
out as a standalone draft to expand into a proper post.*
