# Mixed types behind one trait: `Box<dyn Trait>`

The traits chapter introduced trait objects (`dyn Trait`) and ended on a puzzle: a `dyn Trait` doesn't have a known size at compile time (different implementors are different sizes), so the compiler won't let you put one directly in a `Vec` or return one from a function.
The fix is to put it behind a pointer, and the *owned* pointer is `Box<dyn Trait>`.

```rust
let pipeline: Vec<Box<dyn Command>> = vec![
    Box::new(Uppercase),
    Box::new(Append { suffix: "!".to_string() }),
];
```

Every entry in the vector is one box, one pointer wide, all the same size.
Each box owns whatever concrete type it wraps.
Dropping the vector drops the boxes, which drops the inner values.
This is the exact same pattern the env-file parser chapter uses as `Box<dyn Error>`: "some value, I don't care which concrete type, just give me one owned thing that implements the trait."

Calling a method on a `Box<dyn Command>` looks like calling it on the concrete type: `cmd.run(input)`.
Under the hood, Rust does a *vtable lookup* (the same trick C++ uses for virtual methods) to pick the right implementation.
The cost is one extra indirection per call; the benefit is the heterogeneity above.

## What you're building

A tiny text-transformation pipeline.
The trait is one method:

```rust
trait Command {
    fn run(&self, input: &str) -> String;
}
```

Three commands are already implemented for you:

- `Uppercase` upper-cases the input.
- `Reverse` reverses the input.
- `Append { suffix }` appends a configured suffix.

The exercise is the orchestrator: `apply_pipeline` threads an input string through every command in order, feeding each command's output into the next command's input, and returns the final result.
An empty pipeline returns the input unchanged.

The reason this works is `Box<dyn Command>`.
The pipeline can mix `Uppercase` (a unit struct), `Reverse` (also a unit struct), and `Append { suffix: String }` (carries a field) in the same `Vec`, because each one is hidden behind the same fat pointer.
A generic `Vec<C>` where `C: Command` would only let you pick *one* concrete command type per pipeline.

## Useful from the standard library

- A `for` loop over `&[Box<dyn Command>]` yields `&Box<dyn Command>` on each iteration.
  Method calls auto-deref through the box (and through the `&`), so `cmd.run(...)` just works.
- The pipeline is a *fold*: start with the input, and at each step the next command takes the previous output.
  A plain `let mut current = input.to_string();` plus reassignment in the loop is the most readable thing.
- `str::chars().rev().collect::<String>()` is one way to reverse a string (it's already written in the `Reverse` impl below).
  Note that this reverses by Unicode scalar value, not by grapheme; the tests stick to ASCII so it doesn't matter here.
