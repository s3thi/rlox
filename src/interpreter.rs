use crate::error::{RLoxError, RLoxResult};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;

use rustyline::Editor;

use std::fs;

pub fn run_file(path: String) -> RLoxResult<()> {
    let src = fs::read_to_string(&path)?;
    run(src)?;
    Ok(())
}

pub fn run_prompt() -> RLoxResult<()> {
    let mut rl = Editor::<()>::new();

    loop {
        let line = rl.readline("rlox> ")?;
        rl.add_history_entry(line.to_string());
        match run(line) {
            Err(err @ RLoxError::Source { .. }) => eprintln!("{}", err),
            err @ Err(_) => return err,
            Ok(_) => (),
        };
    }
}

fn run(src: String) -> RLoxResult<()> {
    let scanner = Scanner::new(src);
    let tokens: Result<Vec<Token>, RLoxError> = scanner.collect();
    // println!("{:?}", tokens.unwrap());
    let mut parser = Parser::new(tokens.unwrap());
    let ast = parser.parse();
    println!("{}", ast.pretty_print());
    Ok(())
}
