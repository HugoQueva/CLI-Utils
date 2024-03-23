use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::{fs::{remove_dir, remove_file}, io::Error};
use std::path::PathBuf;

pub fn delete(command: &Command, application: &mut Application) -> Result<CommandResult, CommandError> {
    let args = command.args.split_whitespace().collect::<Vec<&str>>();

    let mut file_name = String::new();
    
    for i in 1..args.len() {
        if i == 0 { continue; 

        if i == args.len() - 1 {
            file_name.push_str(&args[i]);
            break;
        }

        file_name.push_str(&format!("{} ", args[i]));
    }

    let path_buffer = PathBuf::from(format!("{}/{}", application.get_working_directory(), file_name));

    let result: Result<(), Error>;

    if path_buffer.is_file() {
        result = remove_file(path_buffer.as_path());
    } else {
        result = remove_dir(path_buffer.as_path());
    }

    match result {
        Ok(_) => {
            return Ok(CommandResult::from_str(&*format!("Deleted {}.", file_name)));
        },
        Err(e) => {
            return Err(CommandError::from_str(&*format!("Could not delete {}! {}", file_name, e)));
        }
    }
}