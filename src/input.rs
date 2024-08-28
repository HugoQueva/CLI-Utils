use crate::command::{Command, CommandType, CommandError};
use std::io::Stdin;

pub fn handle_user_input(stdin: &Stdin) -> Result<Command, CommandError> {
    let mut buffer = String::new();

    let line = stdin.read_line(&mut buffer);
    
    match line {
        Ok(_) => {
            let mut buffer_iterator = buffer.split_whitespace();
            let command = buffer_iterator.next();
            
            if let None = command {
                return Err(CommandError::from("You have to use a command")); 
            }

            let command_type = match command.unwrap() {
                "echo" => CommandType::Echo,
                "cat" => CommandType::Cat,
                "ls" => CommandType::Ls,
                "find" => CommandType::Find,
                "grep" => CommandType::Grep,
                "exit" => CommandType::Exit,
                "clear" => CommandType::Clear,
                "cd" => CommandType::Cd,
                "makefile" => CommandType::MakeFile,
                "makedir" => CommandType::MakeDirectory,
                "delete" => CommandType::Delete,
                _ => CommandType::Unknown,
            };

            let args: Vec<String> = buffer_iterator.map(|x| x.to_owned())
                                                   .collect();

            let command = Command {
                command_type: command_type,
                args: args,
            };

            return Ok(command);
        }
        Err(_) => return Err(CommandError::from("Could not read from stdin")),
    } 

}