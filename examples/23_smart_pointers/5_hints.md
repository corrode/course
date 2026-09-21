# Hints

## boxed_sum

1. `*a` reads the `i32` inside the first box.
2. `i32` is `Copy`, so you can read the two integers through their boxes and add
   them.

## expr_tree

### Expr::add

Compare the argument types with the fields of `Add`. Which operation turns an
owned expression into the field type? You already own both children, so cloning
or evaluating them isn't needed.

### Expr::eval

What is the smallest tree you can evaluate without visiting another node? For
an operation node, think about what information you need from its children.

Matching on borrowed `self` borrows the fields too. If a branch returns a
reference when you need an integer, check the binding's type. Method calls can
auto-deref through a borrowed box; you don't need to move its contents out.

## pipeline

### make_pipeline

A box can take ownership of a command created inside the factory. Which
command needs to own the supplied string?

If Rust infers a vector containing only one concrete command type, try an
explicit `Vec<Box<dyn Command>>` annotation. That gives the different boxes a
common destination type.

### apply_pipeline

Check the types at each stage: `run` borrows its input and returns an owned
string. Which string should the next command receive? A loop is fine; consider
the empty pipeline when choosing your starting value.

Call through the `Command` interface rather than checking concrete types. Method
calls work through the borrowed boxes, so the caller can keep its pipeline.
