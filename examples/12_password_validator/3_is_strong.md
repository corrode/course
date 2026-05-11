# Warm-up: `is_strong`

Welcome to the open-ended chapter. The whole exercise revolves around a
`PasswordReport` value: a structured verdict about a password, with a
numeric score, some human-readable feedback, and a coarse strength label.

We start small. Before tackling the actual scoring, get a feel for the
data by implementing the one-line `is_strong` method on `PasswordReport`.
By convention in this exercise, "strong" means the score is at least
`70`.

This step also introduces the shared `PasswordStrength` enum and
`PasswordReport` struct that every later step will reuse (each step
re-declares them so it can stand on its own).
