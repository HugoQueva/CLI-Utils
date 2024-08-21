use std::{env, path::PathBuf};

pub struct Application {
    pub working_directory: PathBuf,
}

impl Application { 
    pub fn new() -> Application {
        Application {
            working_directory: PathBuf::new()
        }
    }

    pub fn get_working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    pub fn set_working_directory(&mut self, working_directory: PathBuf) {
        self.working_directory = working_directory;
    }

    pub fn set_working_directory_to_current_directory(&mut self) {
        let path_buffer = env::current_dir();

        if let Ok(path) = path_buffer {
            self.working_directory = path;
        }
    }
}