use crate::token::Operator;

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Operation(BinOp),
}

impl Expr {
    pub fn evaluate(&self) -> i32 {
        match self {
            Expr::Number(n) => *n,
            Expr::Operation(o) => match o.op {
                Operator::Add => o.left.evaluate() + o.right.evaluate(),
                Operator::Sub => o.left.evaluate() - o.right.evaluate(),
                Operator::Mul => o.left.evaluate() * o.right.evaluate(),
                Operator::Div => o.left.evaluate() / o.right.evaluate(),
                Operator::Mod => o.left.evaluate() % o.right.evaluate(),
            },
        }
    }
}

#[derive(Debug)]
pub struct BinOp {
    op: Operator,
    left: Box<Expr>,
    right: Box<Expr>,
}

impl BinOp {
    pub fn new(op: Operator, left: Expr, right: Expr) -> Self {
        BinOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}
