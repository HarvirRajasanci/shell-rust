use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn echo(arg: &str) {
    let rest = arg
        .split_once(char::is_whitespace)
        .map(|(_, r)| r.trim())
        .unwrap_or("");
    println!("{}", rest);
}

fn main() {
    let mut commands: HashMap<String, fn(&str)> = HashMap::new();
    commands.insert(String::from("echo"), echo as fn(&str));

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let command = input.split_whitespace().next().unwrap_or("");

        if command == "exit" {
            process::exit(0);
        }

        if let Some(func) = commands.get(command) {
            func(input);
        } else {
            println!("{}: command not found", command);
        }
    }
}
