# Smart Pointers

**A pointer can own a value, not just borrow it.**
A reference such as `&T` borrows a value whose owner must keep it alive.
A `Box<T>` owns its value on the heap, so you can move the box or return it from a function without borrowing a local variable.
Dropping the box drops the value and releases its allocation.
No separate `free` or `delete` is needed.

## Why Own a Value through a Pointer?

Most values need no box.
Two situations make the extra indirection useful.

A recursive type cannot contain another whole value of itself inline.
The compiler would need an infinite amount of space for it.
A `Box<Expr>` gives an expression-tree node a fixed-size pointer to an owned child instead.

A collection of different command types needs a common element type.
`Box<dyn Command>` owns any concrete value that implements `Command` and calls its methods through dynamic dispatch.
By contrast, `&dyn Command` only borrows a command owned elsewhere.
Returning newly created commands is a reason to choose the owned form.

You'll start with a small dereferencing warmup, then construct and evaluate a tree, and finally build and borrow a command pipeline.
The exercises focus on `Box`; other smart pointers get a short recognition guide at the end.
