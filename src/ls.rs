use crate::command::{CommandResult, CommandError};
use std::{fs, io};

pub fn list() -> Result<CommandResult, CommandError> {
    let directory = fs::read_dir("./");

    match directory {
        Ok(dir) => {
            let entries = dir
            .map(| res | res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>();

            match entries {
                Ok(entry_vec) => {
                    for entry in entry_vec {
                        println!("{} ({})", entry.display(), entry.capacity());
                    }
                }
                Err(_) => return Err(CommandError(String::from("Could not read directory"))),
            }
        }
        Err(_) => return Err(CommandError::from_str("Could not read directory!")),
    }

    Ok(CommandResult::with_empty_text())   
}