use crate::command::{CommandError, CommandResult};

pub fn clear() -> Result<CommandResult, CommandError> {
    let result = clearscreen::clear();

    match result {
        Ok(_) => Ok(CommandResult::default()),
        Err(_) => Err(CommandError::from("Could not clear the screen."))
    }
}