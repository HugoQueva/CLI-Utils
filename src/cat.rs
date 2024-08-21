use crate::command::{Command, CommandError, CommandResult};
use crate::Application;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn cat(command: Command, application: &Application) -> Result<CommandResult, CommandError> {
    if command.args.is_empty() {
        return Err(CommandError::from("You must provide parameters to execute cat"));
    }

    let file_path = PathBuf::from(application.get_working_directory()).join(&command.args[0]);

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            return Err(CommandError::from(format!("Cannot open file {}!", command.args[0])));
        }
    };

    let mut file_content = String::new();
    if let Err(err) = file.read_to_string(&mut file_content) {
        return Err(CommandError::from(format!("{}",err)));
    }
    

    Ok(CommandResult::from(format!("\n{}",file_content)))
}