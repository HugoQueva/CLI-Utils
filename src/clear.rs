use crate::command::{CommandError, CommandResult};



pub fn clear() -> Result<CommandResult, CommandError> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("\0");

    Ok(CommandResult::with_empty_text())
}