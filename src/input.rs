use crate::command::{Command, CommandType, CommandError};
use std::io::Stdin;

pub fn handle_user_input(stdin: &Stdin) -> Result<Command, CommandError> {
    let mut buffer = String::new();

    let line = stdin.read_line(&mut buffer);
    
    match line {
        Ok(_) => {
            let mut args: Vec<String> = Vec::new();

            for char in buffer.split_whitespace() {
                args.push(char.to_owned());
            }

            if args.is_empty() {
                return Err(CommandError::from("You have to use a command")); 
            }

            args.remove(0);

            let command_type = match &(*args[0]) {
                "echo" => CommandType::ECHO,
                "cat" => CommandType::CAT,
                "ls" => CommandType::LS,
                "find" => CommandType::FIND,
                "grep" => CommandType::GREP,
                "exit" => CommandType::EXIT,
                "clear" => CommandType::CLEAR,
                "cd" => CommandType::CD,
                "makefile" => CommandType::MAKEFILE,
                "makedir" => CommandType::MAKEDIRECTORY,
                "delete" => CommandType::DELETE,
                _ => CommandType::UNKNOWN,
            };

            let command = Command {
                command_type: command_type,
                args: args,
            };

            return Ok(command);
        }
        Err(_) => return Err(CommandError::from("Could not read from stdin")),
    } 

}