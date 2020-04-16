use crate::error::{RLoxError, RLoxResult};
use crate::scanner::Scanner;

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
    for token in scanner {
        println!("{:?}", token);
    }
    Ok(())
}
