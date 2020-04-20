use std::env;
use std::process;

mod ast;
mod error;
mod interpreter;
mod scanner;
mod token;

use error::RLoxError;
use interpreter::{run_file, run_prompt};

fn main() {
    use ast::{ASTNode, BinaryNode, LiteralNode, PrettyPrinter};
    use token::TokenType;

    let l1 = ASTNode::Literal(LiteralNode::new(TokenType::Number(11.33)));
    let l2 = ASTNode::Literal(LiteralNode::new(TokenType::Number(145.21)));
    let expression = ASTNode::Binary(BinaryNode::new(l1, TokenType::Plus, l2));
    println!("{}", expression.pretty_print());
}

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let err = if args.len() > 2 {
//         println!("Usage: rlox [script]");
//         process::exit(64);
//     } else if args.len() == 2 {
//         run_file(args[1].to_string())
//     } else {
//         run_prompt()
//     };

//     match err {
//         Err(err) => match err {
//             RLoxError::IO { .. } => {
//                 eprintln!("IO error while trying to run file: {}", args[1].to_string());
//                 process::exit(1);
//             }
//             src_error @ RLoxError::Source { .. } => {
//                 eprintln!("{}", src_error);
//                 process::exit(65);
//             }
//             RLoxError::Interrupted => {
//                 eprintln!("Interrupted!");
//                 process::exit(1);
//             }
//             RLoxError::EOF => {
//                 eprintln!("Goodbye!");
//                 process::exit(0);
//             }
//         },
//         Ok(()) => (),
//     }
// }
