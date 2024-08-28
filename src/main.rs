mod application;
mod makefile;
mod makedir;
mod command;
mod display;
mod delete;
mod input;
mod clear;
mod echo;
mod find;
mod cat;
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

        if let Ok(command) = command {

            match command.command_type {
                CommandType::Exit => break,
                 _ => {},
            }

            let result = handle_command(command, &mut application);

            if let Ok(result) = result {
                print_result(result, &application);
            } 
            else if let Err(error) = result {
                print_error(error, &application);
            }

        } else if let Err(error) = command {
            print_error(error, &application);
        }
    }
}