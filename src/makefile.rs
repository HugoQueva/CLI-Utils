use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::fs::File;

pub fn make_file(command: &Command, application: &mut Application) -> Result<CommandResult, CommandError>{
    let args = command.args.split_whitespace().collect::<Vec<&str>>();

    if args.len() < 2 {
        return Err(CommandError::from_str("You need to specify a file name."));
    }

    let file_name = args[1];

    let path = format!("{}/{}", application.get_working_directory(), file_name);

    let file = File::create(path);

    match file {
        Ok(_) => {
            return Ok(CommandResult::from_str(&*format!("File {} created.", file_name)));
        },
        Err(e) => {
            return Err(CommandError::from_str(&*format!("Could not create file! {}", e)));
        }
    }
}