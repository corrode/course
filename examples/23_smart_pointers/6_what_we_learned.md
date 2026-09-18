# Wrapping up smart pointers

You used `Box` to put an integer on the heap, then to build a recursive expression tree and a pipeline of different command types.
Boxing the integer was practice; the tree and pipeline give you reasons to use `Box` in your own code.

## What we learned

- Smart pointers such as `Box` and `Rc` manage ownership and drop the value when its last owner is dropped.
  They use RAII to release resources without an explicit `free` or `delete`.
- `Box<T>` is the simplest smart pointer: one owner, one heap allocation, dropped when the box goes out of scope.
  C++ devs: this is `std::unique_ptr<T>`.
- `Box::new(value)` constructs a box.
  `*boxed` dereferences it, and most method calls auto-deref so you rarely need to write `*` by hand.
- Recursive enums need indirection.
  `Add(Expr, Expr)` is infinitely sized; the fields of `Add(Box<Expr>, Box<Expr>)` are two pointers.
  The compiler can lay out the enum, and evaluation follows the tree recursively.
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
  It pairs with `Rc` for graphs and shows up in some testing patterns.

You could also write the loop in `apply_pipeline` with `.fold(...)`: each command receives the previous output and produces the next one.
