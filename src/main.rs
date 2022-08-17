use colored::Colorize;
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
fn main() {
    loop {
        let command = get_command();
        if command.is_empty() {
            continue;
        }

        let mut parts = command.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cb" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
            }
            command => match Command::new(command).args(args).spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                }
                Err(e) => {
                    println!("{}", format!("Error: {e}").bold().on_red())
                }
            },
        }
    }
}

fn get_command() -> String {
    let path = env::var("PWD").expect("PWD not found");
    let crr_dir = path.split("/").last().unwrap();
    let path = format!("{}", crr_dir).bold().yellow().underline();
    let prefix = format!("{}@{}", whoami::username(), whoami::devicename())
        .bold()
        .underline()
        .green();
    print!("{}", format!("{} {} ##>", prefix, path).bold().cyan());
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}
