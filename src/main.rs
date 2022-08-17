use std::{io::{stdout, Write, stdin}, process::Command};

fn main() {
    loop {
        let command = get_command();
        let mut child = Command::new(command).spawn().unwrap();
        child.wait().unwrap();
    }
}

fn get_command() -> String {
    print!("##> ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}
