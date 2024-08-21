use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::{fs, io};
use std::path::PathBuf;

pub fn delete(command: Command, application: &mut Application) -> Result<CommandResult, CommandError> {
    let file_name = &command.args[0];

    let path_buffer = PathBuf::from(application.get_working_directory()).join(&file_name);

    let result: io::Result<()>;

    if path_buffer.is_file() {
        result = fs::remove_file(path_buffer);
    } else {
        result = fs::remove_dir(path_buffer);
    }

    match result {
        Ok(_) => {
            return Ok(CommandResult::from(format!("Deleted {}.", file_name)));
        },
        Err(e) => {
            return Err(CommandError::from(format!("{}", e)));
        }
    }
}