# Wrapping Up Smart Pointers

Boxing an integer was practice.
The tree and pipeline gave you reasons to own values through pointers.

## What We Learned

- `Box<T>` owns a heap value and drops it when the box is dropped.
  `Box::new` constructs a box, `*` dereferences it, and method calls usually auto-deref.
- Recursive types need indirection for a finite layout.
  `Expr::add` moves child expressions into boxes; `eval` borrows the resulting tree without consuming it.
- `Box<dyn Command>` lets a factory return ownership of different concrete command types behind one interface.
  A borrowed `&dyn Command` instead depends on an owner elsewhere.
- Owning commands does not mean consuming them on every run.
  `apply_pipeline` borrows the slice and dispatches through the trait, leaving the pipeline available for reuse.

## Recognizing Other Smart Pointers

These types are for recognition only here, not additional exercise requirements.

- `Rc<T>` provides shared ownership on one thread through reference counting.
  Cloning an `Rc` adds an owner without cloning the inner value; the value is dropped when the last strong owner is gone.
  Strong `Rc` cycles keep their values alive, so use non-owning `Weak<T>` links where a relationship should not keep a value alive.
  Upgrading a `Weak` returns an `Option` because the value may already have been dropped.
- `Arc<T>` uses atomic reference counting for shared ownership across threads.
  It does not make an unsafe-to-share inner value thread-safe or provide mutation by itself.
- `RefCell<T>` allows mutation through a shared reference by checking borrowing rules at runtime.
  Conflicting calls to `borrow` or `borrow_mut` panic; the `try_borrow` variants return errors instead.
  It can pair with `Rc` for shared mutable data on one thread, but it does not prevent reference cycles.
