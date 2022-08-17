use colored::Colorize;
use std::io::{Error, Write};
use std::{env};
use console::{Term, Key};
use std::path::Path;
use std::process::Command;

// Search
// History
fn main() {
    let mut commands: Vec<String> = Vec::new();

    loop {
        let command = get_command(&mut commands);

        match command {
            Action::Down => {
                println!("Got down arrow")
            }
            Action::Up => {
                println!("Got up arrow")
            }

            Action::Command(command) => {
                // commands.push(command.clone());
                // curr_idx+= 1;
                apply_command(&command);
            }
        }

        println!("{:#?}", commands);
    }
}

fn get_command(commands: &mut Vec<String>) -> Action {
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
    let mut idx = commands.len();
    let mut command_choosed: Option<String> = None;

    loop {
        let key = term.read_key().unwrap();
        
        match key {
            Key::ArrowUp => {
                if commands.len() == 0 || idx == 0 {
                    continue;
                }

                if let Some(command) = commands.get(idx - 1) {
                    term.clear_line().unwrap();
                    term.write_all(format!("{} {}", prefix, command).as_bytes(),).unwrap();
                    idx-= 1;
                    command_choosed = Some(command.clone());
                    continue;
                }

                return Action::Up
            },
            Key::ArrowDown => return Action::Down,
            Key::Enter => {
                if let Some(command) = command_choosed {
                    println!("\n");
                    return Action::Command(command)
                }
                commands.push(command.clone());
                break;
            }
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

fn apply_command(input: &str) {
    if input.is_empty() {
        return
    }

    let mut parts = input.split_whitespace();
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
        "exit" => return,
        command => match Command::new(command).args(args).spawn() {
            Ok(mut child) => {
                child.wait().unwrap();
            }
            Err(e) => eprint(e),
        },
    }
}

enum Action {
    Up,
    Down,
    Command(String),
}