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
A `Box<Expr>` is one pointer wide, so the two fields of `Add` have a fixed size.
The enum also needs space to distinguish its variants.
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
Interpreters and calculators often use trees like this to represent expressions.
Parsing text like `"(1 + 2) * 4"` produces an `Expr` tree; evaluating that tree is just walking it.

Your job is the evaluation half: implement `Expr::eval(&self) -> i32` so it returns the numeric value of the whole tree.
You can follow the tree's structure with recursion.
`Num(v)` is the base case (return `*v`, since `self` is borrowed); `Add(l, r)` returns `l.eval() + r.eval()`; `Mul(l, r)` does the same with `*`.

The `match` binds `l` and `r` as `&Box<Expr>`, and method calls auto-deref through the box, so `l.eval()` works directly without `(*l).eval()`.

The tests build the trees for you; no new boxes are needed in `eval`.
There's no tail-call optimization guarantee, but the test trees are tiny.
