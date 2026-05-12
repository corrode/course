# Returning multiple values

Functions in Rust return a single value, but a tuple lets you bundle
several values into that single return. It's the lightest-weight way
to hand back more than one thing without defining a new type.

Here you'll return a `(String, u32)` pair: a name and an age.
