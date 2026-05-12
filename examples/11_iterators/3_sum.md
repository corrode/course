# Summing with an iterator

Iterators were popularised by functional languages like Lisp (created by
John McCarthy in 1958), and today they're a core building block in most
modern languages. Rust's iterators are lazy: they don't do any work
until you ask for a result.

The simplest pattern is to take a sequence and collapse it down to a
single value. You could write a `for` loop with a running total, but
the standard library can do this for you in one call.
