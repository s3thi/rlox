use std::env;
use std::process;

mod error;
mod interpreter;
mod scanner;
mod token;

use error::RLoxError;
use interpreter::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = env::args().collect();

    let err = if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].to_string())
    } else {
        run_prompt()
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
