use crate::{command::{Command, CommandError, CommandResult}, Application};
use std::fs::File;

pub fn make_file(command: &Command, application: &mut Application) -> Result<CommandResult, CommandError>{
    let args = command.args.split_whitespace().collect::<Vec<&str>>();

    if args.len() < 2 {
        return Err(CommandError::from_str("You need to specify a file name."));
    }

    let mut document_name = String::new();

    for i in 1..args.len() {
        if i == 0 { continue; }

        if i == args.len() - 1 {
            document_name.push_str(&args[i]);
            break;
        }

        document_name.push_str(&format!("{} ", args[i]));
    }
    
    let path = format!("{}/{}", application.get_working_directory(), document_name);

    let file = File::create(path);
    
    match file {
        Ok(_) => {
            return Ok(CommandResult::from_str(&*format!("Document {} created.", document_name)));
        },
        Err(e) => {
            return Err(CommandError::from_str(&*format!("Could not create file! {}", e)));
        }
    }
}