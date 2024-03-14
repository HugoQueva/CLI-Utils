use crate::command::{CommandError, CommandResult};

pub fn clear() -> Result<CommandResult, CommandError> {
    // ! FIXME: This implementation does not clear the terminal,
    // ! but changes the cursor position to the beginning of the terminal.
    /*
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("\0");
    */

    Err(CommandError::from_str("This command is not implemented yet!"))
}