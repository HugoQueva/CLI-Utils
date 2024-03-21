mod application;
mod makefile;
mod makedir;
mod command;
mod display;
mod input;
mod clear;
mod echo;
mod ls;
mod cd;

use std::io::{self};
use command::{handle_command, CommandType};
use display::{print_error, print_result};
use input::handle_user_input;
use application::*;

fn main(){
    let stdin = io::stdin();

    let mut application = Application::new();
    application.set_working_directory_to_current_directory();

    loop {

        let command = handle_user_input(&stdin);

        match command {
            Ok(command) => {

                match command.command_type {
                    CommandType::EXIT => break,
                    _ => {},
                }

                let result = handle_command(command, &mut application);

                match result {
                    Ok(result) => print_result(result, &application),
                    Err(error) => print_error(error, &application),
                }
            },
            Err(error) => print_error(error, &application),
        }
    }
}