# Wrapping up

Ownership is the model underneath the moves, borrows, and references you've been using throughout the course.

## The whole picture

- Every value has one owner and is dropped when the owner leaves scope.
  That alone gives you automatic cleanup with no garbage collector and no double-frees.
- Moving transfers ownership; `Copy` types duplicate instead.
  You met this with `String` (moves) and `i32` (copies) back in the moves chapter.
- Borrowing (`&T`, `&mut T`) lets you use a value without owning it, under the aliasing rule: many shared borrows or one mutable borrow, never both.
  That rule is what turns data races into compile errors.
- Lifetimes are the same guarantee seen from the reference's side: a borrow can't outlive what it points at.
  You rarely write them by hand early on.

## Why it's worth the friction

C++ uses RAII and destructors for deterministic cleanup, while garbage-collected languages track which values are still alive at runtime.
Rust adds compile-time ownership and borrowing rules, so safe Rust turns use-after-free, double-free, and data races into compile errors.
The borrow checker can be frustrating while you're still learning its rules, but that trade, an argument with the compiler now instead of a crash later, is the core bet the language makes.
