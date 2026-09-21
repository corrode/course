# Functions

If you’ve ever gone through customs, you know the importance of declaring what you’re bringing in.
Rust takes that idea a step further: you declare both **what goes in and what comes out** of a function: its parameters and return type.

A function body is a block expression, which means the last expression in its body is its return value.
That means you can skip the `return` keyword and just leave off the semicolon on the last expression, which looks pretty nice.

## Anatomy

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
Reading left to right: `fn` says "this is a function", `add` is its name, the
parentheses list the parameters with their types, and `-> i32` declares the
return type. Every parameter type and the return type are spelled out
explicitly. Rust never guesses these for you.

To call a function, write its name with the arguments in parentheses:

```rust
let sum = add(2, 3);   // sum: i32 = 5
```

## Expressions, Not Statements

The body of a function is a *block*: zero or more statements followed by an
optional final expression. When that final expression has no trailing semicolon,
its value becomes the value of the block. For a function body, that block value
is also the function's return value.

```rust
fn double(n: i32) -> i32 {
    n * 2          // no semicolon: this is the return value
}
```

You can also use an explicit `return`, which is occasionally useful for early
exits, but the no-semicolon form is more idiomatic for the final value:

```rust
fn double(n: i32) -> i32 {
    return n * 2;  // works too, but unusual at the end
}
```

That semicolon thing trips up newcomers. The rule is short: a semicolon turns an
expression into a statement (which has no value). Leave the semicolon off the
final expression when you want to return its value. Adding one accidentally
turns the body into "do this, then return `()`" and the compiler will complain
that the types don't match. In the first exercise, you'll make that error
disappear by changing a single character.

## A Few Good Habits

- Start with the smallest function you can. Single-purpose functions are easier
  to test and easier to read.
- Use descriptive parameter names: `width: u32`, not `w: u32`.
- Prefer the *least demanding* parameter type that still lets you do the job. If
  you only need to *read* a string, take `&str`, not `String`. Taking `String`
  would force the caller to hand over the value or clone it, while `&str` lets
  them keep it. A `&str` parameter also accepts string literals, borrowed
  `String` values, and slices of larger text buffers without conversion at the
  call site. Borrowing does not clone the string, and moving a `String` would
  transfer ownership without moving its heap allocation.

  The same idea extends to other types, such as `&[T]` instead of `&Vec<T>` and
  `&Path` instead of `&PathBuf`.
