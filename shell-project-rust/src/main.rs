use std::io::{self, Write};
use std::process::{Command, exit};

mod simple_shell;

fn main() {
    let mut cmd = String::new();
    let mut cmd_tokens: Vec<String> = Vec::new();
    let shell = simple_shell::SimpleShell::new();

    print!("tsh> ");
    io::stdout().flush().unwrap();

    while io::stdin().read_line(&mut cmd).unwrap() > 0 {
        if !cmd.trim().is_empty() {
            shell.parse_command(&cmd, &mut cmd_tokens);
            if shell.is_quit(&cmd_tokens) {
                exit(0);
            } else {
                shell.exec_command(&cmd_tokens);
            }
        }
        cmd.clear();
        cmd_tokens.clear();

        print!("tsh> ");
        io::stdout().flush().unwrap();
    }

    println!();
    exit(0);
}