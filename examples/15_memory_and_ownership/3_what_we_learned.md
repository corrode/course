# Wrapping up

Ownership isn't a single feature you switch on; it's the model underneath everything else in the language.

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

Every other memory model asks you to either manage memory by hand and get it right every single time, or accept a garbage collector's runtime cost.
Rust moves that work to compile time.
The borrow checker can be frustrating while you're still learning its rules, but what you get back is a whole class of bugs (use-after-free, double-free, data races) that simply can't reach production.
That trade, an argument with the compiler now instead of a crash later, is the core bet the language makes.
