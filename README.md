# Rust Workshop Exercises

![Screenshot of the course page](/static/assets/screenshot.jpg)

A hands-on Rust course for working developers. You write small programs,
the compiler keeps you honest, and over a couple of dozen exercises you
end up comfortable with the parts of Rust you'll actually use day to day.

This is the official course repository for [corrode](https://corrode.dev),
a Rust consultancy that helps teams adopt Rust for production.

## Who this is for

You should be comfortable with at least one mainstream language (Python,
JavaScript, Go, Java, C#, Ruby, etc.) and the usual programming basics:
variables, functions, loops, conditionals.

No prior Rust knowledge is assumed.

## How to use it

You'll need [Rust](https://rustup.rs/) installed.

```bash
git clone https://github.com/corrode/course.git
cd course
```

Each file in `examples/` is one exercise. Open it, read the comment at the
top, fill in the `todo!()` bodies, and run its tests:

```bash
cargo test --example 00_hello_rust
```

When the tests pass, move to the next file. That's it.

If you want to try an exercise without installing anything, paste the file
into the [Rust Playground](https://play.rust-lang.org/) and click "Test".

### Optional: progress tracking with the CLI

The repository ships with a small CLI that talks to a course server. Your
instructor will tell you whether to use it. If you're working on your own,
you can skip this section entirely.

```bash
cargo install --path .

cargo course init                                       # register
cargo course submit examples/00_hello_rust.rs           # submit
cargo course submit examples/00_hello_rust.rs --pedantic # earn a star with fmt + clippy
cargo course status                                     # see your progress
```

## The exercises

Each exercise focuses on one concept. Later exercises build on earlier ones,
so it's worth going in order.

| #  | File                          | Topic                                              |
|----|-------------------------------|----------------------------------------------------|
| 00 | `00_hello_rust`               | String creation and formatting                     |
| 01 | `01_integer_handling`         | Arithmetic and number operations                   |
| 02 | `02_strings_and_chars`        | `&str`, `String`, and `char`                       |
| 03 | `03_enums_and_matching`       | Pattern matching with HTTP status codes            |
| 04 | `04_vectors_basics`           | Growable arrays and basic operations               |
| 05 | `05_hashmaps`                 | Key-value storage for configuration and caching   |
| 06 | `06_tuples`                   | Multiple return values and destructuring           |
| 07 | `07_option_handling`          | Safe handling of missing values                    |
| 08 | `08_result_handling`          | Error handling and validation                      |
| 09 | `09_ownership_basics`         | Memory safety and borrowing                        |
| 10 | `10_structs_and_methods`      | User account management                            |
| 11 | `11_iterator_patterns`        | Data transformation with functional patterns       |
| 12 | `12_password_validator`       | Open-ended project, your call                      |
| 13 | `13_question_mark_operator`   | Error propagation with `?`                         |
| 14 | `14_modules`                  | Organising code with `mod` and visibility          |
| 15 | `15_word_counter`             | Strings, vectors, and hashmaps together            |
| 16 | `16_env_parser`               | Parsing `.env` configuration files                 |
| 17 | `17_csv_parser`               | A more involved parsing exercise                   |
| 18 | `18_rust_fundamentals_quiz`   | Final review of what you've learned                |

Every exercise has a module-level comment explaining what you're building
and why, and a set of unit tests you'll make pass.

## How to work through an exercise

1. Read the `//!` comment at the top of the file. It tells you what's
   going on and links to relevant standard library docs.
2. Look at the test cases at the bottom. They show exactly what the
   functions should do.
3. Replace each `todo!()` with real code, one at a time.
4. Run `cargo test --example <name>` until everything passes.
5. Try a variation. Change a test, break the code, see what the compiler
   says. The compiler is the best teacher you'll get.

A few habits that help:

- **Compile often.** `cargo check` is fast. Catching one mistake at a time
  is much easier than catching ten.
- **Read the error messages slowly.** Rust's errors are unusually good.
  Look for the `help:` and `note:` lines.
- **Use the standard library docs.** When you don't know which method
  exists on a `String` or `Vec`, the answer is usually one click away at
  [doc.rust-lang.org/std](https://doc.rust-lang.org/stable/std/).
- **Don't rush.** This isn't a race.

If you get stuck, see [Getting Unstuck](docs/getting_unstuck.md) for
concrete strategies.

## Helpful resources

- [The Rust standard library](https://doc.rust-lang.org/stable/std/) is
  your primary reference. It's well-written and has everything you need.
- [The Rust Book](https://doc.rust-lang.org/book/) is the official
  long-form introduction.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) is a
  good lookup reference for syntax and small patterns.

## For instructors: server setup

The repository also contains a server that powers the CLI's progress
tracking and an admin dashboard. You only need this if you're running
the course for a group.

```bash
cp .env.example .env       # edit the file with your admin token
cargo run                  # starts the server on http://localhost:3000
```

Configuration in `.env`:

```env
# Required: secret token for admin access
CORRODE_ADMIN_TOKEN=your_secret_admin_token_here

# Optional: database location (defaults to ./playground.db)
DATABASE_URL=sqlite:./playground.db

# Optional: port (defaults to 3000)
PORT=3000
```

The admin dashboard is at `http://localhost:3000/admin?token=<your_token>`
and shows participants, their completion status, and recent submissions.

## About corrode

[corrode](https://corrode.dev) is a Rust consultancy. We help teams adopt
Rust for production: training, code reviews, architecture, and the kind
of practical experience you can't get from a book.

If you'd like an in-person workshop, a remote training session, or a
review of your Rust codebase, get in touch.
