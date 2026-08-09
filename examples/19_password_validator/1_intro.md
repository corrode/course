# A creative break

This open-ended project combines structs, enums, iterators, `Option`, `Result`, vectors, and strings.

The files get longer from here, and the in-browser editor may start to feel cramped.
When that happens, you can open the same work in a roomier editor:

- **Open in Web Editor** (the button above each editor) opens the current file on [github.dev](https://github.dev/corrode/course): a full browser-based VS Code with proper find-in-file, multi-cursor, and the keyboard shortcuts you'd expect.
  No install, no clone.
- **Run it locally.** Clone [the repo](https://github.com/corrode/course), open a chapter under `examples/NN_slug/`, and run `cargo test --example NN_slug` (or `cargo check` for a faster compile-only loop).
  Local gets you `rust-analyzer`, on-save formatting, and the proper Rust workflow you'll want once you start writing real projects.

## Patterns you can reuse

**Counting with iterators.** The `.filter(...).count()` combo is a quick way to ask "how many of these match?":

```rust
let digit_count = password.chars().filter(|c| c.is_ascii_digit()).count();
```

**Checking for a character class.** When you only need a yes/no answer ("is there any uppercase letter?"), `.chars().any(...)` stops at the first match:

```rust
fn has_uppercase(password: &str) -> bool {
    password.chars().any(|c| c.is_ascii_uppercase())
}
```

The lowercase, digit, and special-character checks follow the same shape: swap in `is_ascii_lowercase`, `is_ascii_digit`, or `"!@#$%^&*".contains(c)` for the closure body.

**Building a list of feedback messages.** A `Vec<String>` you push into as you check each rule keeps the validator readable:

```rust
let mut feedback = Vec::new();
if password.len() < 8 {
    feedback.push("Use at least 8 characters".to_string());
}
```

**Mapping a score to a category.** A simple `match` on a range works nicely for "weak / medium / strong":

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

**Cycling through characters for the generator.** If you want to avoid external crates, you can use the current time's nanoseconds as a poor person's randomness.
Not cryptographically secure, but enough for an exercise:

```rust
use std::time::{SystemTime, UNIX_EPOCH};
let seed = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos() as usize;
```

For real randomness, the [`rand`](https://docs.rs/rand) crate is the standard answer; out of scope here, but worth knowing it exists.

## Ideas to try

Start with the `is_strong` warm-up, then move on to the generator and scoring engine.
Once those tests pass, take the validator in any direction that sounds interesting:

- Turn the validator's terse feedback (`"too short"`, `"missing digit"`) into friendly advice (`"Add at least 4 more characters"`).
- Detect common passwords, repeated runs (`aaa`, `111`), or keyboard walks (`qwerty`, `123456`) and dock points for them.
- Reward variety: a longer character set or a wider mix of classes earns a higher score.
- Swap the clock-based generator for the `rand` crate and compare.

Take your time with this one.
It's deliberately less guided.
