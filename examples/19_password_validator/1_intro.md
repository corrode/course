# A Creative Break

You've used structs, enums, iterators, `Option`, `Result`, vectors, and strings in guided exercises.
Here you get to decide how to put them together in a password validator.

The files get longer from here, and the in-browser editor may start to feel cramped.
When that happens, you can open the same work in a roomier editor:

- **Open in Web Editor** (the button above each editor) opens the current file on [github.dev](https://github.dev/corrode/course): a full browser-based VS Code with proper find-in-file, multi-cursor, and the keyboard shortcuts you'd expect.
  You don't need to install anything or clone the repo.
- To run it locally, clone [the repo](https://github.com/corrode/course), open a chapter under `examples/NN_slug/`, and run `cargo test --example NN_slug` (or `cargo check` for a faster compile-only loop).
  Working locally gives you `rust-analyzer` and on-save formatting, which help when you're working through a larger project.

## Patterns You Can Reuse

### Counting with Iterators

Use `.filter(...).count()` to ask "how many of these match?":

```rust
let digit_count = password.chars().filter(|c| c.is_ascii_digit()).count();
```

### Checking for a Character Class

When you only need a yes/no answer ("is there any uppercase letter?"), `.chars().any(...)` stops at the first match:

```rust
fn has_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}
```

For the lowercase, digit, and special-character checks, swap in `is_ascii_lowercase`, `is_ascii_digit`, or `"!@#$%^&*".contains(c)` for the closure body.

### Collecting Feedback

You can push messages into a `Vec<String>` as you check each rule:

```rust
let mut feedback = Vec::new();
if password.chars().count() < 8 {
    feedback.push("Use at least 8 characters".to_string());
}
```

### Mapping a Score to a Category

Use a `match` with ranges to classify a score as weak, medium, or strong:

```rust
let strength = match score {
    0..30 => PasswordStrength::Weak,
    30..70 => PasswordStrength::Medium,
    _ => PasswordStrength::Strong,
};
```

The `0..30` here is a *range pattern*.
It's the same `..` syntax you saw in the loops chapter for ranges as values, but used inside a `match` arm to mean "any value in `0..30`."
`..=` (inclusive) works in patterns too.

### Cycling through Characters for the Generator

If you want to avoid external crates, you can use the current time's nanoseconds to vary the generated string.
This isn't cryptographically secure, so keep it strictly for the exercise:

```rust
use std::time::{SystemTime, UNIX_EPOCH};
let seed = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos() as usize;
```

For real randomness, use the [`rand`](https://docs.rs/rand) crate.
We won't cover it here, but it's worth knowing it exists.

## Ideas to Try

Start with the `is_strong` warm-up, then move on to the generator and scoring engine.
Once those tests pass, take the validator in any direction that sounds interesting:

- Turn the validator's terse feedback (`"too short"`, `"missing digit"`) into friendly advice (`"Add at least 4 more characters"`).
- Detect common passwords, repeated runs (`aaa`, `111`), or keyboard walks (`qwerty`, `123456`) and dock points for them.
- Reward variety: a longer character set or a wider mix of classes earns a higher score.
- Swap the clock-based generator for the `rand` crate and compare.

Take your time with this one.
It's deliberately less guided.
