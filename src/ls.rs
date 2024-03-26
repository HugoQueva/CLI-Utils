use byte_prefix::calc_bytes;

use crate::{command::{CommandError, CommandResult}, Application};
use std::{fs, io, time};
use chrono::{DateTime, Datelike, Local};
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
                        let metadata = entry.metadata().unwrap(); // ? File should always have a metadata.
                        let creation_time_local = metadata.created().unwrap_or(time::SystemTime::now());;
                        let date: DateTime<Local> = DateTime::from(creation_time_local);
                        let file_size = metadata.len() as f32;

                        if !entry.is_dir() { 
                            println!("[{}-{:02}-{:02}] {} (~{})",
                                date.year(),
                                date.month(),
                                date.day(),
                                entry.display().to_string().green().bold(), 
                                calc_bytes(file_size)
                            );
                        }
                        else{
                            println!("[{}-{:02}-{:02}] {}",
                                date.year(),
                                date.month(),
                                date.day(),
                                entry.display().to_string().blue().bold(), 
                            );
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