use std::env;

pub struct Application {
    pub working_directory: String,
}

impl Application {
    pub fn new() -> Application {
        Application {
            working_directory: String::new()
        }
    }

    pub fn get_working_directory(&self) -> &String {
        &self.working_directory
    }

    pub fn set_working_directory_to_current_directory(&mut self) {
        let path_buffer = env::current_dir();

        match path_buffer {
            Ok(path) => {
                self.working_directory = path.to_str().unwrap_or("./").to_owned();
            },
            Err(_) => {
                self.working_directory = String::from("./");
            }
        }
    }

    pub fn set_working_directory(&mut self, working_directory: String) {
        self.working_directory = working_directory;
    }
}