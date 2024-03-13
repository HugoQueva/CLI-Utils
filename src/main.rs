//* main.rs
mod command;
mod display;
mod input;
mod clear;
mod echo;
mod ls;

use std::io::{self};
use command::{handle_command, CommandType};
use display::{print_error, print_result};
use input::handle_user_input;

fn main(){
    let stdin = io::stdin();

    loop {
        let command = handle_user_input(&stdin);

        match command {
            Ok(command) => {

                match command.command_type {
                    CommandType::EXIT => break,
                    _ => {},
                }

                let result = handle_command(command);

                match result {
                    Ok(result) => print_result(result),
                    Err(error) => print_error(error),
                }
            },
            Err(error) => print_error(error),
        }
    }
}