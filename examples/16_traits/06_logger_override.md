# Overriding Default Methods

`PlainLogger` used both default methods without changing them. Now you have a
logger that adds a tag, such as `"auth: "`, to every line. Its `log` method is
already implemented, so the defaults work here too: `error("nope")` currently
returns `"auth: [ERROR] nope"`.

## Make Errors Stand Out

Add an `error` method inside the existing `impl Logger for TaggedLogger` block.
Use `[CRITICAL]` instead of `[ERROR]`, so the same call returns
`"auth: [CRITICAL] nope"`. Build the severity-prefixed message and pass it
through the logger's own `log` method so the tag appears exactly once. The tag
and message can be any strings, including empty ones.

Leave the trait and the supplied `log` method unchanged, and don't add a `warn`
method. Warnings should still use the default body, producing lines such as
`"auth: [WARN] slow"`. You only have to override the behavior you want to
change.

## Predict Which Methods Run

Before running the tests, trace `Logger::warn(&logger, "slow")` and
`Logger::error(&logger, "nope")` for a `TaggedLogger` with your override.
Which calls enter a default body, and which enter your implementation? When a
default body calls `self.log`, whose `log` runs?

Does spelling the call as `Logger::error` force the trait's default body, or
still use the override? Check your predictions against the tests and method
bodies. The small edit matters because callers use the trait interface in both
cases.

## Useful from the Standard Library

- [`format!`](https://doc.rust-lang.org/std/macro.format.html) builds a `String`
  that you can pass to another method as a string slice.
