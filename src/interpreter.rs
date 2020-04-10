use crate::scanner::{Scanner, Token};
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

pub struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, path: String) -> io::Result<()> {
        let src = fs::read_to_string(&path)?;
        self.run(src);
        if self.had_error {
            // TODO: fix error handling so this moves to main().
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let mut line = String::new();
        let stdin = io::stdin();
        let stdout = io::stdout();

        loop {
            print!("rlox> ");
            stdout.lock().flush()?;
            stdin.lock().read_line(&mut line)?;
            self.run(line.clone());
            self.had_error = false;
        }
    }

    fn run(&self, src: String) {
        let scanner = Scanner::new(src);
        let tokens = scanner.scan();
        for token in tokens {
            print!("{:?}", token);
        }
        io::stdout().lock().flush().unwrap();
    }

    fn error(&mut self, line: u32, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn report(&mut self, line: u32, location: String, message: String) {
        eprintln!("[{}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
