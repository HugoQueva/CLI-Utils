use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::fs::create_dir;

pub fn make_dir(command: &Command, application: &mut Application) -> Result<CommandResult, CommandError>{
    let args = command.args.split_whitespace().collect::<Vec<&str>>();

    if args.len() < 2 {
        return Err(CommandError::from_str("You need to specify a file name."));
    }

    let file_name = args[1];

    let path = format!("{}/{}", application.get_working_directory(), file_name);

    let file = create_dir(path);
    
    match file {
        Ok(_) => {
            return Ok(CommandResult::from_str(&*format!("Directory {} created.", file_name)));
        },
        Err(e) => {
            return Err(CommandError::from_str(&*format!("Could not create file! {}", e)));
        }
    }
}