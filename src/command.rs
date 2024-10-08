use std::str::FromStr;

use crate::{
    cat::cat, cd::set_cd, clear::clear, delete::delete, echo::echo, find::find, ls::list, makedir::make_dir, makefile::make_file, Application
};

#[derive(Debug)]
pub enum CommandType {
    Echo,
    Cat,
    Ls,
    Find,
    Grep,
    Clear,
    Cd,
    Exit,
    MakeFile,
    MakeDirectory,
    Delete,
    Unknown,
}

pub struct Command {
    pub command_type: CommandType,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct CommandParseError;

pub struct CommandResult {
    pub message: Option<String>,
}

impl FromStr for CommandResult {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CommandResult{
            message: Some(String::from(s)),
        })
    }
}

impl From<String> for CommandResult {
    fn from(value: String) -> Self {
        CommandResult {
            message: Some(value),
        }
    }
}

impl From<&str> for CommandResult {
    fn from(value: &str) -> Self {
        CommandResult {
            message: Some(value.to_string())
        }
    }
}

impl Default for CommandResult {
    // Returns a `CommandResult` object without any message
    fn default() -> Self {
        CommandResult {
            message: None,
        }
    }
}

pub struct CommandError {
    pub message: Option<String>
}

impl From<&str> for CommandError {
    fn from(value: &str) -> Self {
        CommandError {
            message: Some(value.to_string())
        }
    }
}

impl From<String> for CommandError {
    fn from(value: String) -> Self {
        CommandError {
            message: Some(value),
        }
    }
}

impl Default for CommandError {
    // Returns a `CommandError` object without any message
    fn default() -> Self {
        CommandError {
            message: None,
        }
    }
}

//////////////////////////////////////
///            HANDLERS            ///
//////////////////////////////////////

pub fn handle_command(command: Command, application: &mut Application) -> Result<CommandResult, CommandError> {
    match command.command_type {
        CommandType::Unknown => Err(CommandError::from("This command does not exist!")),
        CommandType::Echo => echo(command),
        CommandType::Ls => list(application),
        CommandType::Clear => clear(),
        CommandType::Cd => set_cd(command, application),
        CommandType::MakeFile => make_file(command, application),
        CommandType::MakeDirectory => make_dir(command, application),
        CommandType::Delete => delete(command, application),
        CommandType::Cat => cat(command, application),
        CommandType::Find => find(command),
        _ => Ok(CommandResult::from("This command is not yet implemented!")),
    }
}