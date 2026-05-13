# Countdown

Time to write a function from scratch. `countdown(n)` should print
the numbers from `n` down to `1`, each on its own line, and then
print `"Liftoff!"`. If `n == 0`, it should print only `"Liftoff!"`.

```text
countdown(3)
3
2
1
Liftoff!
```

This exercise pulls in two new ideas.

## Returning nothing

Not every function hands back a value. `countdown` exists for its
*side effect* (printing), so there's nothing meaningful to return.
In Rust, that means the return type is `()`, the unit type, an empty
tuple. You don't have to write it. A function with no `-> Type` is
implicitly `-> ()`:

```rust
fn greet() {
    println!("hi");
}
// equivalent to:
fn greet() -> () {
    println!("hi");
}
```

`fn main()` is exactly this: a function that returns `()`. You've
been writing one since chapter one.

## Recursion

A function in Rust can call itself. That's **recursion**, and it's
the technique you'll use here instead of a loop.

A recursive function has two parts:

- A **base case**: a condition where the function does *not* call
  itself, and just returns. Without one, you have an infinite loop.
- A **recursive case**: the function does a little work and then
  asks itself to handle a smaller version of the same problem.

For `countdown`, the base case is `n == 0` (just print `"Liftoff!"`).
The recursive case prints `n` and then calls `countdown` with `n - 1`.
Each call gets its own fresh parameters, stacked on top of the
previous one, until the base case unwinds the whole stack.

## Useful from the standard library

- [`println!`](https://doc.rust-lang.org/std/macro.println.html)
  prints a line. You've already used it in `main`. It's a macro
  (note the `!`), but you call it just like a function.
