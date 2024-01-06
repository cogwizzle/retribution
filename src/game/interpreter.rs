use crate::ret_lang;
use crate::game::state;

/// A function that takes a command runs game logic based on it.
/// 
/// # Arguments
/// * `command` - A reference to a command from the ret_lang module.
///
/// # Returns
/// * `Result<String, &str>` - A string that is the output of the command, or an error message.
fn travel_interpreter<'a>(command: &'a ret_lang::Command) -> Result<String, &'a str> {
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

/// A function that takes a command runs game logic based on it.
///
/// # Arguments
/// * `command` - A reference to a command from the ret_lang module.
/// * `state` - A mutable reference to a GameState.
///
/// # Returns
/// * `Result<String, &str>` - A string that is the output of the command, or an error message.
///
/// # Examples
/// ```
/// use retribution::game;
/// use retribution::game::interpreter;
/// use retribution::ret_lang;
/// use retribution::game::state;
///
/// let mut game_state = state::GameState::new();
/// game_state.mode = state::Mode::Travel;
/// let command = ret_lang::parse_input("go north").unwrap_or_else(|e| panic!("{}", e));
/// let output = interpreter::interpreter(&command, &mut game_state).unwrap_or_else(|e| panic!("{}", e));
/// assert_eq!(output, "Hero went north");
/// ```
pub fn interpreter<'a>(command: &'a ret_lang::Command, state: &mut state::GameState) -> Result<String, &'a str> {
    match state.mode {
        state::Mode::Travel => travel_interpreter(command),
        _ => Err("Not able to do that action right now."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the travel_interpreter function.
    #[test]
    fn travel_interpreter_test() {
        let command = ret_lang::parse_input("go north").unwrap_or_else(|e| panic!("{}", e));
        let output = travel_interpreter(&command).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(output, "Hero went north");
    }

    // Test the travel_interpreter function with an invalid command.
    #[test]
    fn travel_interpreter_invalid_command_test() {
        let command = ret_lang::parse_input("endure").unwrap_or_else(|e| panic!("{}", e));
        let output = travel_interpreter(&command);
        assert_eq!(output, Err("Not able to do that action right now."));
    }
}
