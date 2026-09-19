# Hints

## boxed_sum

1. `*a` reads the `i32` inside the first box.
2. `i32` is `Copy`, so you can read the two integers through their boxes and add
   them.

## expr_tree

### Expr::add

1. The arguments are owned `Expr` values, but the `Add` fields require
   `Box<Expr>`.
2. Use `Box::new` on each argument and put the resulting boxes in `Self::Add`,
   preserving their order.

### Expr::eval

1. Match on `self` with arms for `Self::Num`, `Self::Add`, and `Self::Mul`.
2. Because `self` is borrowed, the pattern bindings borrow the fields too.
   Dereference the integer in the `Num` arm to return its value.
3. Evaluate both children recursively and combine their results with the
   appropriate operator. Method calls auto-deref through `&Box<Expr>`, so
   `left.eval()` works without `(*left).eval()`.

## pipeline

### make_pipeline

1. Wrap each concrete command in `Box::new`. Construct `Append` by moving
   `suffix` into its field.
2. Return a `vec!` with the uppercase command first and the append command
   second. The return type tells Rust to convert both boxes to
   `Box<dyn Command>`. If you use a local vector, annotate it as
   `Vec<Box<dyn Command>>` so it does not infer a single concrete command type.

### apply_pipeline

1. Start with an owned string, `let mut current = input.to_string();`.
2. A `for` loop over `commands` borrows each box. Replace `current` with
   `command.run(&current)` on each iteration. Method calls auto-deref through
   the reference and the box.
3. Return `current` after the loop. An empty pipeline leaves that starting
   string unchanged.
