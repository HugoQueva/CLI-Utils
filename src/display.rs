use crate::{command::{CommandError, CommandResult}, Application};
use colored::*;

pub fn print_result(result: CommandResult, application: &Application) {
    if let Some(command_message) = result.message {

        let prefix = "$~".bold().green();
        let arrow = "->".bold().white();
        let working_dir_string = application.get_working_directory().display().to_string().bold().white();
        let formatted_string = format!("{0} {1} {2} {3}",
            prefix ,
            arrow, 
            working_dir_string,
            command_message
        );

        println!("{}", formatted_string);
    }
}

pub fn print_error(error: CommandError, application: &Application) {
    if let Some(command_message) = error.message {
        
        let prefix = "$~".bold().red();
        let arrow = "->".bold().white();
        let working_dir_string = application.get_working_directory().display().to_string().bold().white();
        let formatted_string = format!("{0} {1} {2} {3}",
            prefix ,
            arrow, 
            working_dir_string,
            command_message.red()
        );
    
        println!("{}", formatted_string);
    }
}