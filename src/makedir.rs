use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::fs::create_dir;
use std::path::PathBuf;

pub fn make_dir(command: Command, application: &mut Application) -> Result<CommandResult, CommandError>{
    if command.args.is_empty() {
        return Err(CommandError::from("You need to specify a file name."));
    }

    let file_name: String = command.args
                                         .iter()
                                         .map(|arg| format!("{}", arg))
                                         .collect();

    let path = PathBuf::from(application.get_working_directory()).join(&file_name);

    let file = create_dir(path);
    
    match file {
        Ok(_) => {
            return Ok(CommandResult::from(&*format!("Directory {} created.", file_name)));
        },
        Err(e) => {
            return Err(CommandError::from(&*format!("Could not create file! {}", e)));
        }
    }
}