use std::env;

mod error;
mod expr;
mod lexer;
mod parser;
mod token;

fn main() {
    let args = env::args().collect::<Vec<String>>()[1..].join(" ");

    let tokens = lexer::tokenize(&args);
    if let Err(e) = tokens {
        println!("{}\n{}^\n{}", args, vec![" "; e.index].join(""), e);
        return;
    }

    let expr = parser::parse(tokens.unwrap());
    if let Err(e) = expr {
        println!("{}", e);
        return;
    }

    println!("{}", expr.unwrap().evaluate());
}
