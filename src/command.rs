use crate::{
    cd::set_cd,
    clear::clear,
    delete::delete, 
    echo::echo, 
    ls::list, 
    makedir::make_dir, 
    makefile::make_file, 
    Application
};

#[derive(Debug)]
pub enum CommandType {
    ECHO,
    CAT,
    LS,
    FIND,
    GREP,
    CLEAR,
    CD,
    EXIT,
    MAKE_FILE,
    MAKE_DIRECTORY,
    DELETE,
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
}

//////////////////////////////////////
///            HANDLERS            ///
//////////////////////////////////////

pub fn handle_command(command: Command, application: &mut Application) -> Result<CommandResult, CommandError> {
    match command.command_type {
        CommandType::UNKNOWN => Err(CommandError::from_str("This command does not exist!")),
        CommandType::ECHO => echo(command),
        CommandType::LS => list(application),
        CommandType::CLEAR => clear(),
        CommandType::CD => set_cd(command, application),
        CommandType::MAKE_FILE => make_file(&command, application),
        CommandType::MAKE_DIRECTORY => make_dir(&command, application),
        CommandType::DELETE => delete(&command, application),
        _ => Ok(CommandResult::from_str("This command is not yet implemented!")),
    }
}