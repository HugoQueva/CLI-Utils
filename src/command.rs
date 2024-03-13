use crate::{clear::clear, echo::echo, ls::list};

#[derive(Debug)]
pub enum CommandType {
    ECHO,
    CAT,
    LS,
    FIND,
    GREP,
    CLEAR,
    EXIT,
    UNKNOWN,
}

pub struct Command {
    pub command_type: CommandType,
    pub args: String,
}

pub struct CommandResult(pub String);

impl CommandResult {
    pub fn from_str(s: &str) -> CommandResult {
        CommandResult(String::from(s))
    }

    pub fn with_empty_text() -> CommandResult {
        CommandResult(String::new())
    }
}

pub struct CommandError(pub String);

impl CommandError {
    pub fn from_str(s: &str) -> CommandError {
        CommandError(String::from(s))
    }

    pub fn with_empty_text() -> CommandError {
        CommandError(String::new())
    }
}

//////////////////////////////////////
///            HANDLERS            ///
//////////////////////////////////////

pub fn handle_command(command: Command) -> Result<CommandResult, CommandError> {
    match command.command_type {
        CommandType::UNKNOWN => Err(CommandError::from_str("This command does not exist!")),
        CommandType::ECHO => echo(command),
        CommandType::LS => list(),
        CommandType::CLEAR => clear(),
        
        _ => Ok(CommandResult::from_str("This command is not yet implemented!")),
    }
}