use std::env;

mod token;
mod error;
mod lexer;

fn main() {
    let args = env::args().collect::<Vec<String>>()[1..].join(" ");

    let tokens = lexer::tokenize(&args);
    if let Err(e) = tokens {
        println!("{}\n{}^\n{}", args, vec![" "; e.index].join(""), e);
        return;
    }

    println!("{:?}", tokens);
}
