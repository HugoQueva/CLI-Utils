use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::path::PathBuf;

pub fn set_cd(command: Command, application: &mut Application) -> Result<CommandResult, CommandError> {
    if command.args.is_empty() {
        return Ok(CommandResult::from(application.get_working_directory().to_str().unwrap_or("~")));
    }

    let target_directory = &command.args[0];

    let path_buffer = PathBuf::from(target_directory);

    if !path_buffer.exists() {
        return Err(CommandError::from("This directory does not exist!"));
    }
    else if !path_buffer.is_dir() {
        return Err(CommandError::from("The path must be a directory!"));
    }

    application.set_working_directory(path_buffer);

    Ok(CommandResult::default())
}