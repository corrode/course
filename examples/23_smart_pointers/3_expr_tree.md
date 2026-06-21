# A recursive type that needs `Box`

Try to imagine this enum without the `Box`:

```rust
enum Expr {
    Num(i32),
    Add(Expr, Expr),   // a node holds two whole sub-expressions inline
    Mul(Expr, Expr),
}
```

The compiler has to decide how many bytes one `Expr` occupies.
`Add` is at least two `Expr`s, each of which is at least two `Expr`s, which is... you see the problem.
The size is infinite, and the compiler refuses to lay out the type.

`Box<Expr>` fixes it.
A `Box` is always one pointer wide, regardless of what it points to, so the compiler now knows that `Add` is two pointers' worth of memory.
The actual sub-expressions live on the heap, reached through those pointers.

```rust
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
```

This is the same trick C uses with `struct node { struct node *l; struct node *r; }` and that Java/C# get for free because every object is already a reference.
Rust just wants you to ask for the indirection explicitly.

## What you're building

`Expr` is a tiny *expression tree*: a value is either a literal number, the sum of two sub-expressions, or the product of two sub-expressions.
This is how every interpreter and every calculator represents code internally.
Parsing text like `"(1 + 2) * 4"` produces an `Expr` tree; evaluating that tree is just walking it.

Your job is the evaluation half: implement `Expr::eval(&self) -> i32` so it returns the numeric value of the whole tree.
Recursion mirrors the data perfectly.
`Num(v)` is the base case (just return `v`); `Add(l, r)` returns `l.eval() + r.eval()`; `Mul(l, r)` does the same with `*`.

The `match` gives you a borrow of each inner `Box<Expr>`, and method calls auto-deref through the box, so `l.eval()` works directly without `(*l).eval()`.

## Useful from the standard library

- `Box::new(Expr::Num(2))` builds a leaf you can put on either side of an `Add` or `Mul`.
  The tests wire larger trees together for you.
- Pattern matching on a reference: `match self { Expr::Add(l, r) => ... }` binds `l: &Box<Expr>` and `r: &Box<Expr>`.
  Method calls auto-deref, so `l.eval()` is fine; `*l` would reach the inner `Expr` if you ever needed it directly.
- Recursion in Rust works exactly like recursion anywhere else.
  There's no tail-call optimization guarantee, but the test trees are tiny.
