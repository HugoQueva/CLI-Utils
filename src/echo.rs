use crate::command::{Command, CommandResult, CommandError};

// ! Could of just use a println! macro, but I wanted to keep the code clean
// ! and experience with macros.
/// Print a string to the terminal output.
/// 
///  # Exemples
/// 
/// ```
/// echo!("Hello, world!");
/// ```
/// 
/// ```
/// echo!(String::from("Hello, world!"));
/// ```
/// 
///  # Errors
/// 
/// Will return an error or an empty string if the input parameter 
/// is not a valid **`String`** or **`&str`**
/// 
///  # Note
/// This macro does not support the **``std::fmt::Debug``** trait.
macro_rules! echo {
    ($x: expr) => {
        println!("{}", $x);
    };
}

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

    echo!(echo_string);

    Ok(CommandResult::with_empty_text())
}