use colored::Colorize;
use std::env;
use std::io::{stdin, stdout, Write, Error};
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
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprint(e);
                }
            }
            command => match Command::new(command).args(args).spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                }
                Err(e) => eprint(e)
            },
        }
    }
}

fn get_command() -> String {
    let path = env::current_dir().unwrap().display().to_string();
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


fn eprint(e: Error) {
    println!("{}", format!("Error: {e}").bold().on_red())
}