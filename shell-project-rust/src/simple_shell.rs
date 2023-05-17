use std::io::{self, Write};
use std::process::Command;

pub struct SimpleShell;

impl SimpleShell {
    pub fn new() -> Self {
        SimpleShell
    }

    pub fn parse_command(&self, cmd: &str, cmd_tokens: &mut Vec<String>) {
        // TODO: Tokenize the command string into arguments
        let tokens: Vec<&str> = cmd.trim().split_whitespace().collect();
        for token in tokens {
            cmd_tokens.push(token.to_string());
        }
    }

    pub fn exec_command(&self, cmd_tokens: &[String]) {
        // TODO: Execute the command
        let cmd = &cmd_tokens[0];
        let args = &cmd_tokens[1..];
        let output = Command::new(cmd).args(args).output();

        match output {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    io::stdout().write_all(&output.stdout).unwrap();
                }
                if !output.stderr.is_empty() {
                    io::stderr().write_all(&output.stderr).unwrap();
                }
            }
            Err(error) => {
                eprintln!("Error executing command: {}", error);
            }
        }
    }

    pub fn is_quit(&self, cmd_tokens: &[String]) -> bool {
        // TODO: Check for the command "quit" that terminates the shell
        if let Some(cmd) = cmd_tokens.get(0) {
            cmd == "quit"
        } else {
            false
        }
    }
}