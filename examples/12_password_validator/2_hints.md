# Hints

This chapter is open-ended. The hints below are scaffolding, not a
solution. They're here to keep you moving when you're stuck on *where
to start*, not on *which trick to use*.

## Where to start

1. Implement `PasswordReport::is_strong` first. It's a one-liner: `self.score >= 70`.
2. Implement `PasswordValidator::validate` with the **base requirements
   only** (length + uppercase + lowercase + digit + special). Skip the
   advanced ideas until the four base tests pass.

## Skeleton for `validate`

```rust
fn validate(password: &str) -> PasswordReport {
    let mut score: u8 = 0;
    let mut feedback: Vec<String> = Vec::new();

    // 1. length checks (each adds to score; failure adds feedback)
    // 2. character-class checks (uppercase/lowercase/digit/special)
    // 3. compute strength from score
    // 4. construct PasswordReport { input: password.to_string(), ... }

    todo!()
}
```

## Useful one-liners

- Length: `password.len()` or, more correctly, `password.chars().count()`.
- Has uppercase: `password.chars().any(|c| c.is_ascii_uppercase())`.
- Has digit: `password.chars().any(|c| c.is_ascii_digit())`.
- Has special: `password.chars().any(|c| "!@#$%^&*".contains(c))`.

## Strength enum

```rust
let strength = match score {
    0..=29 => PasswordStrength::Weak,
    30..=69 => PasswordStrength::Medium,
    _      => PasswordStrength::Strong,
};
```

## On `PasswordGenerator::generate_secure_password`

The exercise text suggests using
`SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos()` as a
source of variability. That's enough to pass the test; in real code,
reach for the [`rand`](https://docs.rs/rand) crate.
