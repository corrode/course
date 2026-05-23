# Functions

The elephant in the room: you've been writing a function since chapter
one. Every `fn main() { ... }` is one. And every `println!(...)` is a
*call*, though `println!` is a macro, which is why it has the `!`. So
this chapter isn't really introducing functions. It's introducing the
parts of `fn` we've quietly skipped: parameters, return types, the
body, and a few small Rust-specific quirks.

## Anatomy

The shape is always the same:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Reading left to right: `fn` says "this is a function", `add` is its
name, the parentheses list the parameters with their types, and `-> i32`
declares the return type. Every parameter type and the return type are
spelled out explicitly. Rust never guesses these for you.

To call a function, write its name with the arguments in parentheses:

```rust
let sum = add(2, 3);   // sum: i32 = 5
```

## Expressions, not statements

The body of a function is a *block*, and a block is one or more
statements followed by an optional final expression. The final
expression (if there is one, and it has no trailing semicolon) becomes
the value of the block, which becomes the return value of the function.

```rust
fn double(n: i32) -> i32 {
    n * 2          // no semicolon: this is the return value
}
```

You can also use an explicit `return`, which is occasionally useful for
early exits, but the no-semicolon form is more idiomatic for the final
value:

```rust
fn double(n: i32) -> i32 {
    return n * 2;  // works too, but unusual at the end
}
```

That semicolon thing trips up newcomers. The rule is short: a
semicolon turns an expression into a statement (which has no value).
Forgetting one at the end of the function is the *correct* thing to do
when you want the value to be returned. Adding one accidentally turns
the body into "do this, then return `()`" and the compiler will
complain that the types don't match. The first exercise lets you feel
that error first-hand.

The three exercises after that each pull at one more thread: returning
nothing, returning a value built recursively, and modifying a parameter
inside the body.

## A few good habits

- Start with the smallest function you can. Single-purpose functions
  are easier to test and easier to read.
- Use parameter names that say what the value *is*, not what type it
  is: `width: u32`, not `w: u32`.
- Prefer the *least demanding* parameter type that still lets you do
  the job. "Least demanding" means: ask the caller for as little as
  possible. If you only need to *read* a string, take `&str`, not
  `String`. Three reasons this matters:
  1. Taking `String` would force the caller to hand over their value (or
     `.clone()` it). Taking `&str` lets them keep it.
  2. A `&str` parameter accepts string literals (`"hi"`), borrows of owned
     strings (`&my_string` coerces from `&String` to `&str`), and slices of
     larger buffers, all without conversion at the call site.
  3. A `&str` is just a pointer and a length; passing one costs nothing. A
     `String` parameter would mean moving (or cloning) a heap buffer on every
     call.

  The same idea extends to other types: take `&[T]` instead of
  `&Vec<T>`, `&Path` instead of `&PathBuf`, and so on. We'll come
  back to this pattern in the vectors and ownership chapters.
