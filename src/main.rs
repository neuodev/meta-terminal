use std::io::{stdout, Write, stdin};
use std::process::Command;
use colored::Colorize;

fn main() {
    loop {
        let command = get_command();
        match Command::new(command).spawn() {
            Ok(mut child) => {
                child.wait().unwrap();
            },
            Err(e) => {
                println!("Error: {}", format!("{e}").bold().on_red())
            }
        }
        
    }
}

fn get_command() -> String {
    let prefix = format!("{}@{}", whoami::username(), whoami::devicename()).bold().underline().green();
    print!("{}", format!("{} ##>", prefix).bold().cyan());
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}
