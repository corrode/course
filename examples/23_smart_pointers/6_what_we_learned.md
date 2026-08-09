# Wrapping up smart pointers

The examples use `Box` for a heap-allocated integer, a recursive expression tree, and different command types stored behind `Box<dyn Command>`.

## What we learned

- A **smart pointer** is a type that owns heap data and runs cleanup automatically on drop.
  It's RAII: the destructor releases the resource, so you never write `free` or `delete`.
- `Box<T>` is the simplest smart pointer: one owner, one heap allocation, dropped when the box goes out of scope.
  C++ devs: this is `std::unique_ptr<T>`.
- `Box::new(value)` constructs a box.
  `*boxed` dereferences it, and most method calls auto-deref so you rarely need to write `*` by hand.
- Recursive enums need indirection.
  `Add(Expr, Expr)` is infinitely sized; `Add(Box<Expr>, Box<Expr>)` is two pointers.
  The compiler can lay it out, and recursion mirrors the data exactly.
  The same concept underpins parsers, interpreters, and ASTs everywhere.
- `Box<dyn Trait>` is the owned form of a trait object.
  It lets one vector own different concrete types behind a shared interface, just as `Box<dyn Error>` held different error types in the env-file parser.
- Dynamic dispatch through a trait object costs one vtable lookup per call.
  For this small command pipeline, one lookup per stage is unlikely to matter.
  Reach for generics (`fn f<T: Command>`) when you want the compiler to monomorphize away the indirection.

## Other smart pointers, briefly

- `Rc<T>` ("reference counted") gives you multiple owners on a single thread.
  The value is dropped when the last `Rc` goes away.
  C++ analogue: `std::shared_ptr<T>` without the atomic overhead.
- `Arc<T>` uses atomic reference counting for shared ownership across threads.
  The value inside still has to be safe to share between threads.
- `RefCell<T>` provides *interior mutability*: borrow checking moves from compile time to runtime, so you can mutate through a shared reference.
  It pairs with `Rc` for graph-shaped data and shows up in some testing patterns.
  You can go a long way without needing it.

## Connections to earlier chapters

The explicit loop in `apply_pipeline` has the same shape as `.fold(...)`: each command receives the previous output and produces the next one.
`Box<dyn Error>` in the env-file parser uses the same owned trait-object pattern to hold different error types.
