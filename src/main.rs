use std::env;
use std::process;

mod interpreter;
mod scanner;

use interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if interpreter.run_file(args[1].to_string()).is_err() {
            println!("Error running file");
            process::exit(1);
        }
    } else {
        if interpreter.run_prompt().is_err() {
            println!("Error reading from standard input");
            process::exit(1);
        };
    }
}
