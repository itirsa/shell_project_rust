# Shell-Project-Rust

My shell project implementation in rust for my operating systems class.

The Simple Shell project is a basic implementation of a command-line shell in Rust. It provides a simple interface for executing commands, handling pipes, and managing the execution flow.

## Features

- Tokenizes command strings into arguments
- Executes commands with or without input
- Handles pipes for command chaining
- Supports built-in commands like `ls` and `pwd`
- Provides a command `quit` to terminate the shell

## Design

The Simple Shell project follows a modular design with the following components:

- `SimpleShell`: The main struct representing the shell. It handles command parsing, execution, and termination.
- `parse_command`: A method that tokenizes a command string into individual arguments.
- `exec_command`: A method that executes a command. If a pipe symbol (`|`) is present, it splits the command into two parts and executes them sequentially.
- `execute_command`: A method that executes a command using the `Command` struct from the `std::process` module.
- `execute_command_with_input`: A method that executes a command with input, handling stdin, stdout, and stderr redirection.
- `is_quit`: A method that checks if the command is the built-in `quit` command to terminate the shell.
- `ls`: A built-in command that lists files and directories in the current directory.

## Usage

To use the Simple Shell, follow these steps:

1. Build the project using `cargo build`.
2. Run the executable using `cargo run`.

Once the shell is running, you can enter commands and execute them. For example:
  $ ls
  total 1320
  rwxr-xr-x 5 itirsayar staff 160 May 16 22:29 .
  drwxr-xr-x 6 itirsayar staff 192 May 16 20:53 ..
  -rwxr-xr-x 1 itirsayar staff 661464 May 16 22:29 main
  -rw-r--r-- 1 itirsayar staff 749 May 16 22:27 main.rs
  -rw-r--r-- 1 itirsayar staff 4238 May 16 22:29 simple_shell.rs

To terminate the shell, enter the `quit` command.

## Limitations and Future Improvements

- The Simple Shell project is a basic implementation and may not support all advanced shell features.
- Error handling and input validation could be improved for robustness.
- Additional built-in commands and shell functionalities could be added in the future.

## License

This project is licensed under the [MIT License](LICENSE).
