use colored::Colorize;

use crate::command::{Command, CommandResult, CommandError};

const ECHO_BOLD: i32 = 0x1 << 1;
const ECHO_RED: i32 = 0x1 << 2;

pub fn echo(command: Command) -> Result<CommandResult, CommandError> {
    if command.args.is_empty() {
        return Err(CommandError::from("You need to specify an argument to echo!"));
    }

    let mut extra_args = 0x1;

    command.args
           .iter()
           .filter(|x| x.starts_with("-"))
           .for_each(|x| {
            let arg = x.as_str();

            match arg {
                "-b" => extra_args |= ECHO_BOLD,
                "-r" => extra_args |= ECHO_RED,
                _ => {},
            };
           });

    let echo_string: String = command.args
                                         .iter()
                                         .filter(|x| !x.starts_with("-"))
                                         .map(|arg| format!("{}", arg))
                                         .collect();


    match extra_args {
        _ if (extra_args & ECHO_BOLD == ECHO_BOLD) && (extra_args & ECHO_RED == ECHO_RED) => println!("{}", echo_string.bold().red()),
        _ if extra_args & ECHO_BOLD == ECHO_BOLD => println!("{}", echo_string.bold()),
        _ if extra_args & ECHO_RED == ECHO_RED => println!("{}", echo_string.red()),
        _ => println!("{}", echo_string),
    }

    Ok(CommandResult::default())
}