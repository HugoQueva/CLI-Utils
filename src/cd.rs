use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::path::PathBuf;

pub fn set_cd(command: Command, application: &mut Application) -> Result<CommandResult, CommandError> {
    let args = command.args.split_whitespace().collect::<Vec<&str>>();
    
    if args.len() < 2 {
        return Err(CommandError::from_str("You need to specify a directory."));
    }

    let target_directory = args[1];

    let path_buffer = PathBuf::from(target_directory);

    if !path_buffer.exists() {
        return Err(CommandError::from_str("This directory does not exist!"));
    }else if !path_buffer.is_dir() {
        return Err(CommandError::from_str("The path must be a directory!"));
    }

    application.set_working_directory(path_buffer.to_str().unwrap_or("./").to_owned());

    Ok(CommandResult::with_empty_text())
}