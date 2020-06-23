mod lexer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tokens: Vec<String> = match lexer::tokenize(args[1].clone()) {
        Ok(tokens) => tokens,
        Err(err) => panic!("{:?}", err)
    };
    println!("Hello, {:?}!", tokens);

}
