# Conditionals and loops

You've already seen `if` and `for` in passing.
Now we'll slow down and look at them on purpose, along with the other two loop forms (`while` and `loop`) and the keywords that control them (`break` and `continue`).

## `if` / `else` / `else if`

The usage is unsurprising:

```rust
if x > 0 {
    println!("positive");
} else if x < 0 {
    println!("negative");
} else {
    println!("zero");
}
```

You might notice two things:

- The condition is a `bool`.
  No truthy strings, no zero-as-false, no parentheses required around the condition.
- The whole `if` is itself an *expression*.
  You can use it on the right-hand side of a `let` binding:

  ```rust
  let label = if x >= 0 { "non-negative" } else { "negative" };
  ```

  Both branches have to produce the same type, and there's no trailing semicolon on the value-producing expression in each branch (just like a function body, see the functions chapter).

## `for` loops

A `for` loop can walk through a range of numbers, the elements of an array, or the items in a collection.
Rust supports all of these through iterators, but you do not need to understand iterators yet to use the loop.
For example:

```rust
for i in 0..5 {            // 0, 1, 2, 3, 4
    println!("{i}");
}

for word in ["hi", "rust"] {
    println!("{word}");
}
```

`0..5` is a *range*: a value that produces the integers from `0` up to (but not including) `5`.
The inclusive form is `0..=5`, which also yields `5`.
Ranges also work as patterns in `match`, such as `1..=10 => ...`.

For larger collections, you'll usually iterate over a `Vec`, a slice, a `HashMap`, or the result of `s.chars()`.
For now, "anything you can put on the right of `for x in ...`" is enough.

## `while` and `loop`

`while` runs as long as a condition is true:

```rust
let mut n = 10;
while n > 0 {
    println!("{n}");
    n -= 1;
}
```

`loop` runs forever, until you `break` out of it.
It is useful when the exit condition is not a simple boolean check at the top, or when you only know whether to stop after doing some work:

```rust
let mut attempts = 0;
loop {
    attempts += 1;
    if try_connect() { break; }
    if attempts > 10 { break; }
}
```

`loop` can also produce a value: pass an expression to `break` and the whole `loop` evaluates to it.

```rust
let answer = loop {
    let guess = read_guess();
    if guess == 42 { break guess; }
};
```

## `break` and `continue`

Both keywords control the *innermost* loop:

- `break` exits the loop immediately.
- `continue` skips the rest of the current iteration and starts the next one.

```rust
for n in 0..10 {
    if n % 2 == 1 { continue; }   // skip odd
    if n > 6     { break; }       // stop at 8
    println!("{n}");              // 0, 2, 4, 6
}
```

## Picking the right loop

When you need to pick one:

- Use `for` when you know what you're iterating over (a range, a slice, a map, the chars of a string).
- Use `while` when the exit condition is a simple "keep going while X is true".
- Use `loop` only when neither of the above fits, usually because the exit condition is in the middle of the body.

If you are unsure, start with `for` when there is already a collection or range to walk through.
Ranges, slices, and collections all fit this syntax because each can produce an iterator.
