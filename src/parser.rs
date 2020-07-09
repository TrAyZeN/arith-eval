use crate::token::{Token, Operator};
use crate::error::{Error, ErrorKind};
use crate::expr::{Expr, BinOp};

struct ExpressionStack {
    stack: Vec<Expr>,
}

impl ExpressionStack {
    fn new() -> Self {
        ExpressionStack { stack: Vec::new() }
    }

    fn push(&mut self, expr: Expr) {
        self.stack.push(expr);
    }

    fn pop(&mut self) -> Option<Expr> {
        self.stack.pop()
    }

    fn push_operation(&mut self, operator: Operator) -> Result<(), Error> {
        let right = match self.pop() {
            Some(e) => e,
            None => return Err(Error::new(ErrorKind::MissingOperand, 0)),
        };

        let left = match self.pop() {
            Some(e) => e,
            None => return Err(Error::new(ErrorKind::MissingOperand, 0)),
        };

        self.push(Expr::Operation(BinOp::new(
            operator,
            left,
            right
        )));

        Ok(())
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, Error> {
    let mut operator_stack: Vec<Token> = Vec::new();
    let mut expression_stack = ExpressionStack::new();

    'outer: for token in tokens {
        match token {
            Token::Number(n) => expression_stack.push(Expr::Number(n)),
            Token::Op(op) => {
                while !operator_stack.is_empty() {
                    match operator_stack.pop().unwrap() {
                        Token::Op(prev_op) => {
                            if prev_op.precedence() < op.precedence() {
                                operator_stack.push(Token::Op(prev_op));
                                break;
                            }

                            expression_stack.push_operation(prev_op)?;
                        },
                        op => {
                            operator_stack.push(op);
                            break;
                        },
                    }
                }

                operator_stack.push(Token::Op(op));
            },
            Token::LParen => operator_stack.push(Token::LParen),
            Token::RParen => {
                while !operator_stack.is_empty() {
                    match operator_stack.pop().unwrap() {
                        Token::Op(op) => expression_stack.push_operation(op.clone())?,
                        Token::LParen => continue 'outer,
                        t => return Err(Error::new(ErrorKind::UnexpectedToken(Token::RParen, t), 0)),
                    }
                }

                if operator_stack.is_empty() {
                    return Err(Error::new(ErrorKind::MissingOperand, 0));
                }
            }
        }
    }

    while let Some(Token::Op(prev_op)) = operator_stack.pop() {
        expression_stack.push_operation(prev_op.clone())?;
    }

    match expression_stack.pop() {
        Some(expr) => Ok(expr),
        None => Err(Error::new(ErrorKind::MissingExpression, 0)),
    }
}
