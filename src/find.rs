use std::io::Error;
use std::{fs, path::PathBuf};
use colored::Colorize;

use crate::command::{Command, CommandError, CommandResult};

fn find_traverse_path(path: &PathBuf, target: &str) -> Result<(), Error> {
    for entry in  fs::read_dir(path)? {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_dir() {
                find_traverse_path(&path,target)?;
            }
            else if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name == target {
                        println!("{}", path.display().to_string().bold().green());
                        return Ok(());
                        }
                    }
                }
        }
    }

    Ok(())
}

pub fn find(command: Command) -> Result<CommandResult, CommandError> {
    if command.args.is_empty() {
        return Err(CommandError::from("You must specify parameters to find command"))
    }

    let root_path = if cfg!(target_os = "windows") {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    };

    let _ = find_traverse_path(&root_path, &command.args[0]);

    Ok(CommandResult::default())
}