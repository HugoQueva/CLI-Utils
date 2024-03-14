use byte_prefix::calc_bytes;

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
                        if !entry.is_dir() { 
                            let metadata = entry.metadata();                       
                            let mut file_size: u64 = 0; //NOTE: To see if the metadata can be read
    
                            match metadata {
                                Ok(meta) => {
                                    file_size = meta.len();
                                },
                                Err(_) => file_size = 0,
                            }

                            // * TODO (Hugo Quéva): Change declaration of `size` to `u64` though the precision 
                            // * of the `f32` higher.
                            let byte_format = calc_bytes(file_size as f32);

                            println!("{} ({})", entry.display(), byte_format);
                        }
                        else{
                            println!("{}", entry.display());
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