use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::fs::File;
use std::path::PathBuf;

pub fn make_file(command: Command, application: &mut Application) -> Result<CommandResult, CommandError>{
    if command.args.is_empty() {
        return Err(CommandError::from("You need to specify a file name."));
    }

    let document_name: String = command.args
                                         .iter()
                                         .map(|arg| format!("{}", arg))
                                         .collect();
    
    let path = PathBuf::from(application.get_working_directory()).join(&document_name);

    let file = File::create(path);
    
    match file {
        Ok(_) => {
            return Ok(CommandResult::from(format!("Document {} created.", document_name)));
        },
        Err(e) => {
            return Err(CommandError::from(format!("Could not create {}!", e)));
        }
    }
}