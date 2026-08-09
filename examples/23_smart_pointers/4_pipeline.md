# Mixed types behind one trait: `Box<dyn Trait>`

When we worked with traits, `dyn Trait` left us with one puzzle: different implementors have different sizes, so the compiler won't let you put a trait object directly in a `Vec` or return one from a function.
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
You've already seen the same pattern as `Box<dyn Error>` in the env-file parser: "some value, I don't care which concrete type, just give me one owned thing that implements the trait."

Calling a method on a `Box<dyn Command>` looks like calling it on the concrete type: `cmd.run(input)`.
Under the hood, Rust does a *vtable lookup* (the same trick C++ uses for virtual methods) to pick the right implementation.
You pay one extra indirection per call in exchange for storing different concrete types in one vector.

## What you're building

You'll build a tiny text-transformation pipeline.
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

Your job is the orchestration: `apply_pipeline` threads an input string through every command in order, feeding each command's output into the next command's input, and returns the final result.
An empty pipeline returns the input unchanged.

Because each command sits behind `Box<dyn Command>`, the same `Vec` can hold `Uppercase`, `Reverse`, and `Append { suffix: String }` even though their concrete types have different sizes.
A generic `Vec<C>` where `C: Command` would only let you pick *one* concrete command type per pipeline.

## Useful from the standard library

- A `for` loop over `&[Box<dyn Command>]` yields `&Box<dyn Command>` on each iteration.
  Method calls auto-deref through the box (and through the `&`), so `cmd.run(...)` just works.
- The pipeline is a *fold*: start with the input, and at each step the next command takes the previous output.
  A plain `let mut current = input.to_string();` plus reassignment in the loop is the most readable thing.
- `str::chars().rev().collect::<String>()` is one way to reverse a string (it's already written in the `Reverse` impl below).
  Note that this reverses by Unicode scalar value, not by grapheme; the tests stick to ASCII so it doesn't matter here.
