use crate::error::RLoxResult;
use crate::scanner::Scanner;
use std::fs;
use std::io::{self, BufRead, Write};

pub fn run_file(path: String) -> RLoxResult<()> {
    let src = fs::read_to_string(&path)?;
    run(src)?;
    Ok(())
}

pub fn run_prompt() -> RLoxResult<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        print!("rlox> ");
        stdout.lock().flush()?;

        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;

        // When running a REPL, we don't want to return an error to
        // the main function. We want to reset the error state and move on.
        run(line.clone()).unwrap();
    }
}

fn run(src: String) -> RLoxResult<()> {
    let mut scanner = Scanner::new(src);
    let tokens = scanner.scan()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
