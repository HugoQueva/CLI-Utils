use crate::command::{Command, CommandResult, CommandError};

pub fn echo(command: Command) -> Result<CommandResult, CommandError> {
    if command.args.is_empty() {
        return Err(CommandError::from("You need to specify an argument to echo!"));
    }

    let echo_string: String = command.args
                                         .iter()
                                         .map(|arg| format!("{}", arg))
                                         .collect();


    println!("{}", echo_string);

    Ok(CommandResult::default())
}