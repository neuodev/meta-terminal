use colored::Colorize;
use std::io::{Error, Write};
use std::time::Duration;
use std::{env, process, thread};
use console::{Term, Key};
use std::path::Path;
use std::process::Command;

// Search
// History
fn main() {
    loop {
        let command = get_command();

        match command {
            Action::Down => {
                println!("Got down arrow")
            }
            Action::Up => {
                println!("Got up arrow")
            }

            Action::Command(command) => {
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
                    "exit" => break,
                    command => match Command::new(command).args(args).spawn() {
                        Ok(mut child) => {
                            child.wait().unwrap();
                        }
                        Err(e) => eprint(e),
                    },
                }
            }
        }
    }
}

fn get_command() -> Action {
    let mut term = Term::stdout();
    let path = env::current_dir().unwrap().display().to_string();
    let crr_dir = path.split("/").last().unwrap();
    let path = format!("{}", crr_dir).bold().yellow().underline();
    let prefix = format!("{}@{}", whoami::username(), whoami::devicename())
        .bold()
        .underline()
        .green();
    let prefix =  format!("{} {} ##>", prefix, path).bold().cyan();

    term.write(format!("{}", prefix).as_bytes()).unwrap();

    let mut command = String::new();
    loop {
        let key = term.read_key().unwrap();
        
        match key {
            Key::ArrowUp => return Action::Up,
            Key::ArrowDown => return Action::Down,
            Key::Enter => {break;}
            Key::Char(c) => {
                command.push(c);
                term.write_all(&String::from(c).as_bytes()).unwrap();
            }
            _ => {}
        }
    }
    println!("\n");
    Action::Command(command)
}

fn eprint(e: Error) {
    println!("{}", format!("Error: {e}").bold().on_red())
}

enum Action {
    Up,
    Down,
    Command(String),
}