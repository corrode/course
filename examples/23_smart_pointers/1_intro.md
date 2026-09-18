# Smart pointers

*A pointer that cleans up after itself walks into a scope.
Nothing leaks.*

A **smart pointer** is a type that owns a value on the heap and runs cleanup automatically when it goes out of scope.
The "smart" part is the cleanup: there's no `free`, no `delete`, no `dispose()`.
When the owning binding drops, the destructor runs and the memory comes back.

If you've used C++, you already know the idea.
`Box<T>` is Rust's `std::unique_ptr<T>`: one owner, freed on drop.
`Rc<T>` is `std::shared_ptr<T>`: shared ownership via a reference count.
Both patterns are an instance of **RAII** (Resource Acquisition Is Initialization): tie the lifetime of a resource to the lifetime of a stack value, and let scope exit do the cleanup.

If your background is Java, Python, or JavaScript, the comparison is trickier.
A `String` in Java is a reference to a heap object that the garbage collector reclaims when no one is looking at it anymore.
Those references are not smart pointers in the Rust sense: there is no single *owner*, and you don't know when (or whether) cleanup happens.
Rust's smart pointers give you the heap allocation without the GC, because ownership tells the compiler exactly when to drop.

## Where `Box` earns its keep

1. `Box<T>` gives you a heap allocation with a single owner.
   You give it a value, it puts it on the heap, and it frees it when the box goes out of scope.
2. Recursive types need indirection.
   A linked-list node that contains *another* node would be infinitely sized; `Box` gives the compiler a fixed-size handle to put in the struct.
3. You met `dyn Trait` earlier; `Box<dyn Trait>` gives the trait object an owner.
   A trait object like `dyn Shape` doesn't have a known size, so it has to live behind a pointer.
   `Box<dyn Shape>` is the owned form, and it's what lets you store a `Vec` of mixed concrete types that all implement the same trait.
   You saw the same trick with `Box<dyn Error>` in the env-file parser.

## Recognizing `Rc` and `RefCell`

You'll use `Box` in the exercises.
I'll also cover `Rc` and `RefCell` briefly so you can recognize them when you read other people's Rust.

### `Rc<T>`: shared ownership, single-threaded

`Box<T>` has exactly one owner.
Sometimes several parts of your program need to share ownership of the same value, and you can't predict which one will be the last to let go.
`Rc<T>` ("reference counted") tracks the number of owners and drops the value when the count hits zero.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);   // bumps the count to 2, no deep copy
let c = Rc::clone(&a);   // count is now 3
// When a, b, and c all go out of scope, the String is dropped.
```

`Rc` is single-threaded.
`Arc<T>` ("atomically reference counted") provides shared ownership across threads, as long as the value inside is itself safe to share between threads.
C++ devs: `Rc` is `shared_ptr` without the atomic overhead, `Arc` is `shared_ptr` with it.

### `RefCell<T>`: interior mutability

The borrow checker normally enforces "one mutable reference or many immutable ones" at compile time.
`RefCell<T>` moves that check to *runtime*: you can hand out an `&RefCell<T>`, and someone holding it can still mutate the inner value by calling `.borrow_mut()`.
If the borrowing rules are violated, the program panics instead of failing to compile.

If you're coming from Java, this is close to a field with a private setter: outside code holds an immutable handle to the object, but the object can still mutate itself.
You don't need `RefCell` for these exercises.
It pairs with `Rc` to build graphs, and it shows up in some testing patterns.
For now, recognizing the name is enough.
