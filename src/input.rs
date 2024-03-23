use crate::command::{Command, CommandType, CommandError};
use std::io::Stdin;

pub fn handle_user_input(stdin: &Stdin) -> Result<Command, CommandError> {
    let mut buffer = String::new();

    let line = stdin.read_line(&mut buffer);
    
    match line {
        Ok(_) => {
            let args = buffer.split_whitespace().collect::<Vec<&str>>();

            if args.len() == 0 {
                return Err(CommandError::from_str("You have to use a command")); 
            }

            let command_type = match args[0] {
                "echo" => CommandType::ECHO,
                "cat" => CommandType::CAT,
                "ls" => CommandType::LS,
                "find" => CommandType::FIND,
                "grep" => CommandType::GREP,
                "exit" => CommandType::EXIT,
                "clear" => CommandType::CLEAR,
                "cd" => CommandType::CD,
                "makefile" => CommandType::MAKE_FILE,
                "makedir" => CommandType::MAKE_DIRECTORY,
                "delete" => CommandType::DELETE,
                _ => CommandType::UNKNOWN,
            };

            let command = Command {
                command_type: command_type,
                args: buffer,
            };

            return Ok(command);
        }
        Err(_) => return Err(CommandError(String::from("Could not read from stdin"))),
    } 

}