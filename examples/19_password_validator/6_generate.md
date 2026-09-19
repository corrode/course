# Generate Test Inputs

So far you've checked strings supplied by a caller.
Now work in the other direction: construct sample strings with known properties, so you can try different lengths without typing every input by hand.
This is test-data generation, not random password generation.

Implement `PasswordGenerator::generate_example_password(length)`:

- For lengths below four, return `Err("Need at least 4 characters.")`.
  Four distinct character classes cannot fit into fewer than four positions.
- Otherwise, return an ASCII string of exactly the requested length containing at least one uppercase letter, one lowercase letter, one digit, and one character from `!@#$%^&*`.
- Use only characters from those four classes.

Any string meeting the contract is acceptable.
It can be predictable and identical across calls; the tests do not require randomness or a particular order.
The `Result` makes the impossible request visible to the caller instead of silently returning a shorter string or panicking.

Don't call this function to create real passwords.
Its job is to produce fixtures for tests, and a fixture can satisfy every formatting rule while being trivial to guess.

## Useful from the Standard Library

- [`String::new`](https://doc.rust-lang.org/std/string/struct.String.html#method.new) creates an empty output buffer.
- [`String::push`](https://doc.rust-lang.org/std/string/struct.String.html#method.push) appends one character.
- `Ok` and `Err` let the caller distinguish a generated string from an unsupported length.
