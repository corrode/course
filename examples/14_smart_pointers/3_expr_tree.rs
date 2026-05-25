/// A minimal expression tree. A value is either a literal number,
/// the sum of two sub-expressions, or the product of two
/// sub-expressions.
///
/// Without `Box`, this enum would have an infinite size: `Add` and
/// `Mul` would need to contain whole `Expr` values directly, which
/// would need to contain whole `Expr` values, and so on.
///
/// `Box<Expr>` puts each sub-expression on the heap and gives `Add`
/// / `Mul` a fixed-size pointer to it. The compiler is happy.
#[derive(Debug, PartialEq)]
enum Expr {
    Num(i32),
    Add(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
}

impl Expr {
    /// Walk the tree and compute its numeric value.
    ///
    /// `Num(v)` is the base case (just return `v`).
    /// `Add(l, r)` is `l.eval() + r.eval()`.
    /// `Mul(l, r)` is `l.eval() * r.eval()`.
    fn eval(&self) -> i32 {
        todo!()
    }
}

#[test]
fn leaf_evaluates_to_its_value() {
    assert_eq!(Expr::Num(7).eval(), 7);
}

#[test]
fn add_two_leaves() {
    // 2 + 3
    let tree = Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)));
    assert_eq!(tree.eval(), 5);
}

#[test]
fn mul_two_leaves() {
    // 4 * 5
    let tree = Expr::Mul(Box::new(Expr::Num(4)), Box::new(Expr::Num(5)));
    assert_eq!(tree.eval(), 20);
}

#[test]
fn nested_mixed_ops() {
    // (1 + 2) * 4 == 12
    let tree = Expr::Mul(
        Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)))),
        Box::new(Expr::Num(4)),
    );
    assert_eq!(tree.eval(), 12);
}

#[test]
fn deeper_tree() {
    // 2 + (3 * (4 + 5))  == 2 + (3 * 9) == 29
    let tree = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Mul(
            Box::new(Expr::Num(3)),
            Box::new(Expr::Add(Box::new(Expr::Num(4)), Box::new(Expr::Num(5)))),
        )),
    );
    assert_eq!(tree.eval(), 29);
}
