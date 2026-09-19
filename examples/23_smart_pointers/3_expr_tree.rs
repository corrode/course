/// An expression tree whose recursive children are owned through boxes.
/// Each box gives the enum a fixed-size field instead of an inline child tree.
#[derive(Debug, PartialEq)]
enum Expr {
    Num(i32),
    Add(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
}

impl Expr {
    /// Own both child expressions in an Add node, preserving their order and shape.
    fn add(left: Self, right: Self) -> Self {
        todo!()
    }

    /// Compute the tree's numeric value without consuming or changing it.
    fn eval(&self) -> i32 {
        todo!()
    }
}

#[test]
fn add_preserves_child_order_and_structure() {
    let left = Expr::Mul(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)));
    let right = Expr::Add(Box::new(Expr::Num(4)), Box::new(Expr::Num(5)));
    let tree = Expr::add(left, right);

    // Check construction without relying on evaluation, which would hide swapped children.
    let expected = Expr::Add(
        Box::new(Expr::Mul(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))),
        Box::new(Expr::Add(Box::new(Expr::Num(4)), Box::new(Expr::Num(5)))),
    );
    assert_eq!(tree, expected);
}

#[test]
fn leaf_evaluates_to_its_value() {
    assert_eq!(Expr::Num(7).eval(), 7);
    assert_eq!(Expr::Num(-3).eval(), -3);
}

#[test]
fn add_two_leaves() {
    let tree = Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)));
    assert_eq!(tree.eval(), 5);
}

#[test]
fn mul_two_leaves() {
    let tree = Expr::Mul(Box::new(Expr::Num(4)), Box::new(Expr::Num(5)));
    assert_eq!(tree.eval(), 20);
}

#[test]
fn nested_mixed_ops() {
    // Evaluate (1 + 2) * 4, with the nested operation on the left.
    let tree = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)))),
        Box::new(Expr::Num(4)),
    );
    assert_eq!(tree.eval(), 12);
}

#[test]
fn asymmetric_tree_can_be_evaluated_again() {
    // Evaluate 2 + (3 * (4 + 5)), with the deeper branch on the right.
    let tree = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Mul(
            Box::new(Expr::Num(3)),
            Box::new(Expr::Add(Box::new(Expr::Num(4)), Box::new(Expr::Num(5)))),
        )),
    );
    assert_eq!(tree.eval(), 29);
    assert_eq!(tree.eval(), 29);
}
