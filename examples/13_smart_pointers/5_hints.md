# Hints

## boxed_sum

1. `*a` reads the `i32` out of the box.
2. `i32` is `Copy`, so reading through the box doesn't move anything
   out. You can use `*a` and `*b` as many times as you want.
3. ```rust
   *a + *b
   ```

## expr_tree

1. Pattern-match on `self`. There are three arms: `Expr::Num`,
   `Expr::Add`, and `Expr::Mul`.
2. `Num(v)` returns `*v` (it's a borrow, so deref to get the `i32`).
   `Add(l, r)` returns `l.eval() + r.eval()`. `Mul(l, r)` is the same
   concept, but for `*`. Method calls auto-deref through the `Box`, so you
   do not need to write `(*l).eval()`.
3. ```rust
   match self {
       Expr::Num(v) => *v,
       Expr::Add(l, r) => l.eval() + r.eval(),
       Expr::Mul(l, r) => l.eval() * r.eval(),
   }
   ```

## pipeline

1. The pipeline is a fold: keep a running `current` string, replace
   it with `cmd.run(&current)` on each iteration, return it at the
   end.
2. An empty pipeline never enters the loop, so `current` ends up as
   the original input. That gets the empty-pipeline test for free.
3. Method calls go through the box automatically. `cmd.run(...)` is
   the only thing you call inside the loop.
4. ```rust
   let mut current = input.to_string();
   for cmd in commands {
       current = cmd.run(&current);
   }
   current
   ```
   Or, if you've peeked at Chapter 15:
   ```rust
   commands
       .iter()
       .fold(input.to_string(), |acc, cmd| cmd.run(&acc))
   ```
