use crate::ret_lang;

/// A function that takes a command runs game logic based on it.
/// 
/// # Arguments
/// * `command` - A command from the ret_lang module.
///
/// # Returns
/// * `Result<String, &str>` - A string that is the output of the command, or an error message.
///
/// # Example
/// **Happy Path**
/// ```
/// use retribution::game::interpreter;
/// use retribution::ret_lang;
///
/// let command = ret_lang::parse_input("go north").unwrap_or_else(|e| panic!("{}", e));
/// let output = interpreter::travel_parser(&command).unwrap_or_else(|e| panic!("{}", e));
/// assert_eq!(output, "Hero went north");
/// ```
/// **Error Path**
/// ```
/// use retribution::game::interpreter;
/// use retribution::ret_lang;
///
/// let command = ret_lang::parse_input("endure").unwrap_or_else(|e| panic!("{}", e));
/// let output = interpreter::travel_parser(&command);
/// assert_eq!(output, Err("Not able to do that action right now."));
/// ```
pub fn travel_parser<'a>(command: &'a ret_lang::Command) -> Result<String, &'a str> {
    match command {
        ret_lang::Command::Go(c) => {
            let output = format!("Hero went {}", c.target);
            Ok(output)
        },
        ret_lang::Command::Exit(_) => {
            std::process::exit(0);
        },
        _ => Err("Not able to do that action right now."),
    }
}
