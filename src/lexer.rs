use crate::error::{Error, ErrorKind};
use crate::token::{Operator, Token};

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let input_chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input_chars.len() {
        let token = match input_chars[i] {
            c if c.is_digit(10) => {
                let mut n = 0;
                while i < input_chars.len() && input_chars[i].is_digit(10) {
                    n = n * 10 + input_chars[i].to_digit(10).unwrap() as i32;
                    i += 1;
                }
                if i < input_chars.len() {
                    i -= 1;
                }

                Some(Token::Number(n))
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let tokens = tokenize("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn number() {
        let tokens = tokenize("42").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42));
    }

    #[test]
    fn expr_no_space() {
        let tokens = tokenize("42+23").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(42));
        assert_eq!(tokens[1], Token::Op(Operator::Add));
        assert_eq!(tokens[2], Token::Number(23));
    }

    #[test]
    fn expr_space() {
        let tokens = tokenize("42  +     23").unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Number(42));
        assert_eq!(tokens[1], Token::Op(Operator::Add));
        assert_eq!(tokens[2], Token::Number(23));
    }

    #[test]
    fn paren_expr() {
        let tokens = tokenize("(42+3)*12").unwrap();
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::LParen);
        assert_eq!(tokens[1], Token::Number(42));
        assert_eq!(tokens[2], Token::Op(Operator::Add));
        assert_eq!(tokens[3], Token::Number(3));
        assert_eq!(tokens[4], Token::RParen);
        assert_eq!(tokens[5], Token::Op(Operator::Mul));
        assert_eq!(tokens[6], Token::Number(12));
    }

    #[test]
    fn invalid_character() {
        let tokens = tokenize("43*a");
        assert!(tokens.is_err());
        assert_eq!(
            tokens,
            Err(Error {
                kind: ErrorKind::InvalidCharacter('a'),
                index: 3
            })
        );
    }
}
