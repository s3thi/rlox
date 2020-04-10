use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if run_file(args[1].to_string()).is_err() {
            println!("Error running file");
            process::exit(1);
        }
    } else {
        if run_prompt().is_err() {
            println!("Error reading from standard input");
            process::exit(1);
        };
    }
}

fn run_file(path: String) -> io::Result<()> {
    let src = fs::read_to_string(&path)?;
    run(src);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut line = String::new();
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        print!("rlox> ");
        stdout.lock().flush()?;
        stdin.lock().read_line(&mut line)?;
        run(line.clone());
    }
}

fn run(src: String) {
    println!("{}", src);
}
