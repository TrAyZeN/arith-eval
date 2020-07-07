#[derive(Debug)]
pub enum Token {
    Number(u32),
    Op(Operator),
    LParen,
    RParen,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
