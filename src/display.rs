use crate::command::{CommandResult, CommandError};
use colored::*;

pub fn print_result(result: CommandResult) {
    if result.0.is_empty() {
         return;
    }

    println!("{} {} {}", "$~".bold().green(), "->".bold().white() , result.0.green());
}

pub fn print_error(error: CommandError) {
    println!("{} {} {}", "$~".bold().red(), "->".bold().white() , error.0.red());
}