use byte_prefix::calc_bytes;

use crate::{command::{CommandError, CommandResult}, Application};
use std::{fs, io};
use colored::*;

pub fn list(application: &Application) -> Result<CommandResult, CommandError> {
    let directory = fs::read_dir(application.get_working_directory());

    match directory {
        Ok(dir) => {
            let entries = dir
            .map(| res | res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>();

            match entries {
                Ok(entry_vec) => {
                    for entry in entry_vec {
                        if !entry.is_dir() { 
                            let metadata = entry.metadata();                       
                            let file_size: f32;
    
                            // * NOTE: Find a way to get the size of the file using a f64 for improved precision.
                            match metadata {
                                Ok(meta) => {
                                    file_size = meta.len() as f32;
                                },
                                Err(_) => file_size = 0.0,
                            }

                            let byte_format = calc_bytes(file_size);

                            println!("{} ({})", entry.display().to_string().green().bold(), byte_format);
                        }
                        else{
                            println!("{}", entry.display().to_string().blue().bold());
                        }
                    }
                }
                Err(_) => return Err(CommandError(String::from("Could not read directory"))),
            }
        }
        Err(_) => return Err(CommandError::from_str("Could not read directory!")),
    }

    Ok(CommandResult::with_empty_text())   
}