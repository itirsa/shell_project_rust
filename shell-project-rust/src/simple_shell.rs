use std::io::{self, Write};
use std::process::{Command, Stdio};

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
      // Check for the pipe symbol ('|') in the command tokens
      if let Some(pipe_index) = cmd_tokens.iter().position(|token| token == "|") {
          // Split the command tokens into two parts: before and after the pipe symbol
          let (cmd_tokens_left, cmd_tokens_right) = cmd_tokens.split_at(pipe_index);

          // Remove the pipe symbol from the command tokens
          let cmd_tokens_left = cmd_tokens_left.iter().filter(|token| *token != "|").cloned().collect::<Vec<String>>();
          let cmd_tokens_right = cmd_tokens_right[1..].to_vec();

          // Execute the left command and capture its output
          let left_output = self.execute_command(&cmd_tokens_left);

          // If the left command execution was successful, use its output as the input for the right command
          if let Ok(output) = left_output {
              self.execute_command_with_input(&cmd_tokens_right, &output.stdout);
          } else {
              eprintln!("Error executing command: {:?}", cmd_tokens_left);
          }
      } else {
          // No pipe symbol found, execute the command normally
          self.execute_command(cmd_tokens);
      }
  }
  fn execute_command(&self, cmd_tokens: &[String]) -> std::io::Result<std::process::Output> {
    // Execute the command and return the output
    let cmd = &cmd_tokens[0];
    let args = &cmd_tokens[1..];
    Command::new(cmd)
        .args(args)
        .output()
}

fn execute_command_with_input(&self, cmd_tokens: &[String], input: &[u8]) {
    // Execute the command with the given input
    let cmd = &cmd_tokens[0];
    let args = &cmd_tokens[1..];
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    // Write the input to the child process's stdin
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input).expect("Failed to write to stdin");
    }

    // Wait for the child process to complete
    child.wait().expect("Failed to wait for command");
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