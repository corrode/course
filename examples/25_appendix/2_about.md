# About This Course

You've made it to the end (or you're just skipping ahead, which is fine too).
With the core exercises behind you, allow me to share a bit of context about
this course and how to keep going.

## Why I Wrote This Course

There's no shortage of Rust resources.

The [official book](https://doc.rust-lang.org/book/) is excellent, and you should read it. So
is [Rust by Example](https://doc.rust-lang.org/rust-by-example/), and plenty of
others.

Nonetheless, there is some ceremony involved in getting started with any of the other resources.
You might have to set up a project, install the Rust toolchain, or figure out how to run tests.
What was missing, for me, was a URL I could just open in a browser and write code immediately.
And you'd get instant feedback on how you're doing.

This course started as a folder on my laptop, the kind of thing I'd pull out
during workshops whenever someone asked "how do I do *this* in Rust?" Over the
years, I collected enough material for a structured path through the language.
This course is the result.

My hope is that it gives you a complementary perspective on Rust, from
a practitioner's perspective. Let me know how I did.

## How to Keep Going

If you haven't done the bonus exercises, please do. They are more free-form and
loose than the strict core exercises, and they are a great way to expand your
skills as you come up with your own challenges.

I recommend [Build a password validator](19_password_validator) as a starting point.

In general, practice the Rust core mechanics on a problem that matters to you.
Some recommendations:

- Build something *tiny*. A CLI that does one thing for you. A scraper. A toy
  interpreter. The smaller and more personal, the better. And I mean it: there
  are no problems too small to practice on. Often, the simplest tasks are the
  most revealing about a language's mechanics.
- Read other people's Rust. Pick a small crate you use and trace one path
  through its source, from a public function to its return value. You do not
  need to understand the whole crate to pick up an idiom or two. After more than
  a decade of Rust, I still pick up a nugget or two from every crate I read.
- Use it for "the next thing" you'd otherwise do in another language. Give
  yourself time; you'll be making decisions that are already familiar in your
  usual language. Decisions about errors, modules, and tests can often be reused
  in later projects, so this is not wasted time.
- Investigate unexpected behavior. When behavior seems surprising or a compiler
  message catches you off guard, take the time to investigate. Reduce it to a
  small example and change one thing at a time. This is a little gift from the
  compiler: it is telling you something about the language that you didn't fully
  grasp.

## A Note from corrode

corrode is a Rust training and consulting company. I help people go from prototype to production.
If your company is looking to migrate to Rust, give me a call and let's see if I can help.
Go to [corrode.dev/services](https://corrode.dev/services/) to learn more.

This course is open source.
You can find the source code at [github.com/corrode/course](https://github.com/corrode/course).
Issues and pull requests are highly appreciated.

Welcome to the Rust community.
