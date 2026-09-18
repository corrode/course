# Mixed Types behind One Trait: `Box<dyn Trait>`

Different `dyn Trait` implementors have different sizes, so the compiler won't let you put a trait object directly in a `Vec` or return one from a function.
The fix is to put it behind a pointer, and the *owned* pointer is `Box<dyn Trait>`.

```rust
let pipeline: Vec<Box<dyn Command>> = vec![
    Box::new(Uppercase),
    Box::new(Append { suffix: "!".to_string() }),
];
```

Every entry in the vector is a `Box<dyn Command>` of the same size.
Unlike `Box<i32>`, it holds both a data pointer and a vtable pointer.
Each box owns whatever concrete type it wraps.
Dropping the vector drops the boxes, which drops the inner values.
The env-file parser uses the same pattern with `Box<dyn Error>`: one owned value of any concrete type that implements the trait.

Calling a method on a `Box<dyn Command>` looks like calling it on the concrete type: `cmd.run(input)`.
Under the hood, Rust does a *vtable lookup* (the same trick C++ uses for virtual methods) to pick the right implementation.
You pay one extra indirection per call in exchange for storing different concrete types in one vector.

## What You're Building

You'll build a tiny text-transformation pipeline.
The trait is one method:

```rust
trait Command {
    fn run(&self, input: &str) -> String;
}
```

Three commands are already implemented for you:

- `Uppercase` upper-cases the input.
- `Reverse` reverses the input by Unicode scalar value, which can separate combining marks from their letters.
- `Append { suffix }` appends a configured suffix.

Implement `apply_pipeline` to pass the input string through every command in order.
Feed each command's output into the next command, then return the final result.
An empty pipeline returns the input unchanged.

Because each command sits behind `Box<dyn Command>`, the same `Vec` can hold `Uppercase`, `Reverse`, and `Append { suffix: String }` even though their concrete types have different sizes.
A generic `Vec<C>` where `C: Command` would only let you pick *one* concrete command type per pipeline.

## Useful from the Standard Library

- A `for` loop over `&[Box<dyn Command>]` yields `&Box<dyn Command>` on each iteration.
  Method calls auto-deref through the box (and through the `&`), so `cmd.run(...)` just works.
- [`ToString::to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string) gives you an owned starting value: `let mut current = input.to_string();`.
  Reassign it after each command.
- [`Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) is an alternative to the loop: use the starting string as the accumulator and pass each command the previous output.
