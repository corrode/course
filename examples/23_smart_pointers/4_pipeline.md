# Owning Different Command Types

A factory cannot return references to commands it creates locally; those commands would be dropped when the factory returns.
`Box<dyn Command>` lets it return ownership instead.
Each box owns a concrete command, while the caller uses the shared `Command` interface.

The supplied trait has one method:

```rust
trait Command {
    fn run(&self, input: &str) -> String;
}
```

Three implementations are provided:

- `Uppercase` uppercases the input.
- `Reverse` reverses Unicode scalar values, which can separate combining marks from their letters.
- `Append { suffix: String }` appends its owned suffix.

A `Vec<C>` with `C: Command` has just one concrete element type.
A `Vec<Box<dyn Command>>` can own different command types in the same collection.
Each element has the same size, holding a data pointer and a vtable pointer for dispatching method calls to the concrete implementation.
The unsized `dyn Command` lives behind the pointer, not directly in the vector.
Dropping the vector drops its boxes and their commands, including any owned strings.

## Build and Run a Pipeline

Implement both functions:

- `make_pipeline(suffix: String) -> Vec<Box<dyn Command>>` returns exactly two commands, first `Uppercase`, then `Append` with the supplied suffix.
  The returned pipeline owns its commands and suffix, so it remains usable after the factory returns.
- `apply_pipeline(commands: &[Box<dyn Command>], input: &str) -> String` passes the input through every command in slice order and returns the final output.
  An empty pipeline returns the input unchanged.
  It must work with any implementation of `Command`, not just the three supplied types.

For a suffix of `"x"`, the factory's pipeline transforms `"hi"` into `"HIx"`, not `"HIX"`.
The factory tests call the returned commands directly, independently of `apply_pipeline`.
The runner tests supply their own pipelines.

## Owning Is Different from Borrowing

`Box<dyn Command>` owns a command; `&dyn Command` borrows one whose owner lives elsewhere.
Here `apply_pipeline` borrows a slice of owned boxes rather than taking the vector away from its caller.
`Command::run` also borrows its command through `&self`.
The caller can therefore run the same pipeline again with another input.
