# Even More CSV Parsing

We've learned about the horror that is CSV parsing in the [State Machines and Stateful Parsing](22_state_machines_and_stateful_parsing) chapter.
And since we can't get enough of it, let's do some more parsing!

This time we try to change the separator without breaking quoted fields and then reuse our parser through a small public API.
Along the way, we learn about Rust's module system.

> [!TIP]
>
> - Each exercise is independent and works in the browser (or as a local Rust test file).
> - The module task includes a working line parser; it doesn't depend on finishing the delimiter task.
> - This bonus chapter does not count toward course completion.
