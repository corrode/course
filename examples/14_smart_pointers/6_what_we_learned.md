# Wrapping up smart pointers

You boxed an integer and added it back out, defined a recursive
expression-tree type that only compiles because of `Box`, and
threaded an input string through a heterogeneous pipeline of
text-transformation commands via `Box<dyn Command>`.

## What we learned

- A **smart pointer** is a type that owns heap data and runs cleanup
  automatically on drop. It's RAII: the destructor releases the
  resource, so you never write `free` or `delete`.
- `Box<T>` is the simplest smart pointer: one owner, one heap
  allocation, dropped when the box goes out of scope. C++ devs:
  this is `std::unique_ptr<T>`.
- `Box::new(value)` constructs a box. `*boxed` dereferences it, and
  most method calls auto-deref so you rarely need to write `*` by
  hand.
- Recursive enums need indirection. `Add(Expr, Expr)` is infinitely
  sized; `Add(Box<Expr>, Box<Expr>)` is two pointers. The compiler
  can lay it out, and recursion mirrors the data exactly. The same
  concept underpins parsers, interpreters, and ASTs everywhere.
- `Box<dyn Trait>` is the owned form of a trait object. It lets you
  store mixed concrete types behind a single interface (a
  `Vec<Box<dyn Command>>` of pipeline stages, all different structs,
  driven through one trait) and underpins the `Box<dyn Error>`
  pattern you'll meet in Chapter 19.
- Dynamic dispatch through a trait object costs one vtable lookup
  per call. That's usually fine. Reach for generics (`fn f<T:
  Command>`) when you want the compiler to monomorphize away the
  indirection.

## Other smart pointers, briefly

- `Rc<T>` ("reference counted") gives you multiple owners on a
  single thread. The value is dropped when the last `Rc` goes away.
  C++ analogue: `std::shared_ptr<T>` without the atomic overhead.
- `Arc<T>` is the same idea but safe to share across threads. It
  shows up when concurrency does.
- `RefCell<T>` provides *interior mutability*: borrow checking moves
  from compile time to runtime, so you can mutate through a shared
  reference. It pairs with `Rc` for graph-shaped data and shows up
  in some testing patterns. You can go a long way without needing
  it.

## Where this goes next

Chapter 16 puts iterators front and center, and you'll see how a
chain of `.iter().fold(...)` could have replaced the explicit loop
in `apply_pipeline`. Chapter 19 brings `Box<dyn Error>` and the
`?` operator together, which is the day-to-day payoff for
understanding `Box<dyn Trait>` here.
