use crate::token::{Token, Operator};
use crate::error::{Error, ErrorKind};

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input_chars.len() {
        let token = match input_chars[i] {
            c if c.is_digit(10) => {
                let mut n = 0;
                while i < input_chars.len() && input_chars[i].is_digit(10) {
                    n = n*10 + input_chars[i].to_digit(10).unwrap();
                    i += 1;
                }
                if i < input_chars.len() {
                    i -= 1;
                }

                Some(Token::Number(n))
            },
            '+' => Some(Token::Op(Operator::Add)),
            '-' => Some(Token::Op(Operator::Sub)),
            '*' => Some(Token::Op(Operator::Mul)),
            '/' => Some(Token::Op(Operator::Div)),
            '%' => Some(Token::Op(Operator::Mod)),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ' ' | '\t' => None,
            c => return Err(Error::new(ErrorKind::InvalidCharacter(c), i)),
        };

        if let Some(t) = token {
            tokens.push(t);
        }

        i += 1;
    }

    Ok(tokens)
}
