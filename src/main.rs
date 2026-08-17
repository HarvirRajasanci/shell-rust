#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        let command = trimmed.split_whitespace().next().unwrap_or("");

        if command == "exit" {
            process::exit(0);
        } else if command == "echo" {
            let rest = trimmed
                .split_once(char::is_whitespace)
                .map(|(_, r)| r.trim())
                .unwrap_or("");
            println!("{}", rest);
        } else {
            println!("{}: command not found", command);
        }
    }
}
