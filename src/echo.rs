use crate::command::{Command, CommandResult, CommandError};

pub fn echo(command: Command) -> Result<CommandResult, CommandError> {
    let args = command.args.split_whitespace().collect::<Vec<&str>>();

    if args.len() < 2 {
        return Err(CommandError::from_str("You need to specify an argument to echo!"));
    }

    let mut echo_string: String = String::new();

    for i in 1..args.len() {
        if i == 0 { continue; }
        
        let arg = args[i];

        echo_string.push_str(&format!("{} ",arg));
    }

    println!("{echo_string}");

    Ok(CommandResult::with_empty_text())
}