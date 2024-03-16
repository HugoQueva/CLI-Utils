use crate::{command::{CommandError, CommandResult}, Application};
use colored::*;

pub fn print_result(result: CommandResult, application: &Application) {
    if result.0.is_empty() {
         return;
    }

    println!("{} {} {} {}",
    "$~ ".bold().green(),
    application.get_working_directory().bold().white(),
     "->".bold().white() 
     , result.0.green());
}

pub fn print_error(error: CommandError, application: &Application) {
    println!("{} {} {} {}", 
    "$~".bold().red(),
    application.get_working_directory().bold().white(),
    "->".bold().white(),
    error.0.red());
}