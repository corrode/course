# A Recursive Type That Needs `Box`

This enum cannot compile:

```rust
// Each recursive field would contain another whole Expr inline.
enum Expr {
    Num(i32),
    Add(Expr, Expr),
    Mul(Expr, Expr),
}
```

The compiler must know how many bytes one `Expr` occupies. Each `Add` would
contain two complete `Expr` values, which could themselves contain more `Expr`
values with no fixed limit. There is no finite layout for this type.

`Box<Expr>` fixes the layout by owning each child through a pointer. A
`Box<Expr>` is one pointer wide, regardless of how large the child's tree
becomes. The enum also needs space to distinguish its variants.

```rust
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
```

Each parent owns its children, rather than borrowing nodes kept alive elsewhere.
Dropping the root drops the owned tree.

## Build and Evaluate a Tree

The supplied `Expr` represents a literal number, a sum, or a product. For
example, an interpreter might represent `(1 + 2) * 4` as a multiplication node
with an addition node on the left and a number on the right.

Implement both methods:

- `Expr::add(left: Self, right: Self) -> Self` takes ownership of two
  expressions and returns an `Add` node containing them in the same left/right
  order. Preserve the child trees rather than replacing them with evaluated
  numbers.
- `Expr::eval(&self) -> i32` returns the numeric value of the tree. It borrows
  the tree, so the same tree can be evaluated again without rebuilding it.
  Evaluation needs no new boxes.

The construction test checks the shape without calling `eval`. The evaluation
tests build their own trees, so you can work on either method independently.
These trees are small; recursive evaluation and dropping are not a strategy for
arbitrarily deep input.
