#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Op(Operator),
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Operator {
    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Add => 2,
            Operator::Sub => 2,
            Operator::Mul => 3,
            Operator::Div => 3,
            Operator::Mod => 3,
        }
    }
}
