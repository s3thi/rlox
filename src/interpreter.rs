use crate::error::RLoxResult;
use crate::scanner::Scanner;
use std::fs;
use std::io::{self, BufRead, Write};

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, path: String) -> RLoxResult<()> {
        let src = fs::read_to_string(&path)?;
        self.run(src)?;
        Ok(())
    }

    pub fn run_prompt(&mut self) -> RLoxResult<()> {
        let mut line = String::new();
        let stdin = io::stdin();
        let stdout = io::stdout();

        loop {
            print!("rlox> ");
            stdout.lock().flush()?;
            stdin.lock().read_line(&mut line)?;

            // When running a REPL, we don't want to return an error to
            // the main function. We want to reset the error state and move on.
            self.run(line.clone()).unwrap();
            self.had_error = false;
        }
    }

    fn run(&self, src: String) -> RLoxResult<()> {
        let scanner = Scanner::new(src);
        let tokens = scanner.scan();
        for token in tokens {
            print!("{:?}", token);
        }
        io::stdout().lock().flush()?;
        Ok(())
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: u32, location: String, message: String) {
        eprintln!("[{}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
