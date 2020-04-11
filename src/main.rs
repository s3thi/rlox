use std::env;
use std::process;

mod error;
mod interpreter;
mod scanner;

use error::RLoxError;
use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    let err = if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        interpreter.run_file(args[1].to_string())
    } else {
        interpreter.run_prompt()
    };

    match err {
        Err(err) => match err {
            RLoxError::IO { .. } => {
                eprintln!("IO error while trying to run file: {}", args[1].to_string());
                process::exit(1);
            }
            src_error @ RLoxError::Source { .. } => {
                eprintln!("{}", src_error);
                process::exit(65);
            }
        },
        Ok(()) => (),
    }
}
