mod lexer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // if args.len() > 1 {
    let tokens: Vec<String> = match lexer::tokenize(args[1].clone()) {
        Ok(tokens) => tokens,
        Err(err) => panic!("{:?}", err)
    };
//         println!("Hello, {:?}!", tokens);
//     } else {
//         println!("No .c file given for compilation");
//         return;
//     }
}
