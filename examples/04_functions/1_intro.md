# Functions

You've been inside a function since the first line you wrote.
`fn main()` is one, and every `println!(...)` is a call (the `!` marks it as a macro).
Functions are already familiar, so the focus is on Rust's explicit parameter and return types, blocks as expressions, and the trailing-semicolon rule that decides what gets returned.

## Anatomy

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Reading left to right: `fn` says "this is a function", `add` is its name, the parentheses list the parameters with their types, and `-> i32` declares the return type.
Every parameter type and the return type are spelled out explicitly.
Rust never guesses these for you.

To call a function, write its name with the arguments in parentheses:

```rust
let sum = add(2, 3);   // sum: i32 = 5
```

## Expressions, not statements

The body of a function is a *block*: zero or more statements followed by an optional final expression.
When that final expression has no trailing semicolon, its value becomes the value of the block.
For a function body, that block value is also the function's return value.

```rust
fn double(n: i32) -> i32 {
    n * 2          // no semicolon: this is the return value
}
```

You can also use an explicit `return`, which is occasionally useful for early exits, but the no-semicolon form is more idiomatic for the final value:

```rust
fn double(n: i32) -> i32 {
    return n * 2;  // works too, but unusual at the end
}
```

That semicolon thing trips up newcomers.
The rule is short: a semicolon turns an expression into a statement (which has no value).
Forgetting one at the end of the function is the *correct* thing to do when you want the value to be returned.
Adding one accidentally turns the body into "do this, then return `()`" and the compiler will complain that the types don't match.
In the first exercise, you'll make that error disappear by changing a single character.

## A few good habits

- Start with the smallest function you can.
  Single-purpose functions are easier to test and easier to read.
- Use parameter names that say what the value *is*, not what type it is: `width: u32`, not `w: u32`.
- Prefer the *least demanding* parameter type that still lets you do the job.
  In other words, ask the caller for as little as possible.
  If you only need to *read* a string, take `&str`, not `String`.
  Taking `String` would force the caller to hand over the value or clone it, while `&str` lets them keep it.
  A `&str` parameter also accepts string literals, borrowed `String` values, and slices of larger text buffers without conversion at the call site.
  Borrowing does not clone the string, and moving a `String` would transfer ownership without moving its heap allocation.

  The same idea extends to other types, such as `&[T]` instead of `&Vec<T>` and `&Path` instead of `&PathBuf`.
  The same pattern applies to vectors and to ownership more broadly.
