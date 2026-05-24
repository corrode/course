# A Creative Break

You might not have noticed yet, but, quietly, like a beautiful magpie collecting
shiny things, you've acquired the skills to write a lot of helpful Rust programs
by now!

This chapter is an open-ended project rather than a focused lesson. You
already have the tools you need: structs, enums, iterators, `Option`,
`Result`, vectors, and strings. The fun part is putting them together.

From this chapter onward the files get longer, and the in-browser
editor starts feeling cramped. You have two upgrades available:

- **Open in Web Editor** (the button above each editor) opens the
  current file on [github.dev](https://github.dev/corrode/course):
  a full browser-based VS Code with proper find-in-file, multi-cursor,
  and the keyboard shortcuts you'd expect. No install, no clone.
- **Run it locally.** Clone
  [the repo](https://github.com/corrode/course), open a chapter under
  `examples/NN_slug/`, and run `cargo test --example NN_slug` (or
  `cargo check` for a faster compile-only loop). Local gets you
  `rust-analyzer`, on-save formatting, and the proper Rust workflow
  you'll want once you start writing real projects.

## A few patterns that come up

**Counting with iterators.** The `.filter(...).count()` combo is a quick
way to ask "how many of these match?":

```rust
let digit_count = password.chars().filter(|c| c.is_ascii_digit()).count();
```

**Building a list of feedback messages.** A `Vec<String>` you push into
as you check each rule keeps the validator readable:

```rust
let mut feedback = Vec::new();
if password.len() < 8 {
    feedback.push("Use at least 8 characters".to_string());
}
```

**Mapping a score to a category.** A simple `match` on a range works
nicely for "weak / medium / strong":

```rust
let strength = match score {
    0..30 => PasswordStrength::Weak,
    30..70 => PasswordStrength::Medium,
    _ => PasswordStrength::Strong,
};
```

The `0..30` here is a *range pattern*. It's the same `..` syntax you saw
in chapter 12 for ranges as values, but used inside a `match` arm to
mean "any value in `0..30`." `..=` (inclusive) works in patterns too.

**Cycling through characters for the generator.** If you want to avoid
external crates, you can use the current time's nanoseconds as a poor
person's randomness. Not cryptographically secure, but enough for an
exercise:

```rust
use std::time::{SystemTime, UNIX_EPOCH};
let seed = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos() as usize;
```

For real randomness, the [`rand`](https://docs.rs/rand) crate is the
standard answer; out of scope here, but worth knowing it exists.

Take your time with this one. It's deliberately less guided.
