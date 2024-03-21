use crate::command::{CommandError, CommandResult};

pub fn clear() -> Result<CommandResult, CommandError> {
    let result = clearscreen::clear();

    match result {
        Ok(_) => Ok(CommandResult::with_empty_text()),
        Err(_) => Err(CommandError::from_str("Could not clear the screen."))
    }
}