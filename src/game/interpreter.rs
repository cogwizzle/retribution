//! # Interpreter
//! A module that contains the interpreter for the game.
use crate::ret_lang;
use crate::game::state;
use crate::game::map;

/// A function that takes a command runs game logic based on it.
/// 
/// # Arguments
/// * `command` - A reference to a command from the ret_lang module.
/// * `state` - A mutable reference to a GameState.
///
/// # Returns
/// * `Result<String, &str>` - A string that is the output of the command, or an error message.
fn travel_interpreter<'a>(command: &'a ret_lang::Command, state: &mut state::GameState) -> Result<String, &'a str> {
    match command {
        ret_lang::Command::Go(c) => {
            let (x, y) = match state.room {
                Some(r) => r,
                None => return Err("Not able to do that action right now."),
            };

            // A function that handles updating the room and returning the output.
            let mut handle_room_change = |new_coords: (i32, i32)| {
                let new_room = match state.map {
                    Some(ref m) => m.get_grid_square(new_coords.0, new_coords.1),
                    None => return Err("Not able to do that action right now."),
                };
                let new_grid_square = match new_room {
                    Some(r) => {
                        state.room = Some(new_coords);
                        r
                    },
                    None => return Err("Not able to do that action right now."),
                };
                match new_grid_square {
                    map::GridSquare::Room(r) => Ok(format!("Hero went {}. {}", c.target, r.description)),
                    _ => return Err("Not able to do that action right now."),
                }
            };
            match c.target.to_lowercase().as_str() {
                "north" => {
                    let new_coords = (x, y - 1);
                    handle_room_change(new_coords)
                },
                "south" => {
                    let new_coords = (x, y + 1);
                    handle_room_change(new_coords)
                },
                "east" => {
                    let new_coords = (x + 1, y);
                    handle_room_change(new_coords)
                },
                "west" => {
                    let new_coords = (x - 1, y);
                    handle_room_change(new_coords)
                },
                _ => return Err("Not able to do that action right now."),
            }
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
/// use retribution::game::interpreter;
/// use retribution::game::map;
/// use retribution::game::state;
/// use retribution::game;
/// use retribution::ret_lang;
///
/// let mut game_state = state::GameState::new();
/// game_state.mode = state::Mode::Travel;
/// let command = ret_lang::parse_input("go north").unwrap_or_else(|e| panic!("{}", e));
/// let output = match interpreter::interpreter(&command, &mut game_state) {
///   Ok(o) => o,
///   Err(e) => e.to_string(),
/// };
/// assert_eq!(output, "Not able to do that action right now.");
/// ```
pub fn interpreter<'a>(command: &'a ret_lang::Command, state: &mut state::GameState) -> Result<String, &'a str> {
    match state.mode {
        state::Mode::Travel => travel_interpreter(command, state),
        _ => Err("Not able to do that action right now."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migration::map;

    /// Test the travel_interpreter function.
    #[test]
    fn travel_interpreter_test() {
        let mut game_state = state::GameState::new();
        let test_map = map::test_area();
        game_state.map = Some(test_map);
        game_state.room = Some((1,1));
        let command = ret_lang::parse_input("go north").unwrap_or_else(|e| panic!("{}", e));
        let output = travel_interpreter(&command, &mut game_state).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(output, "Hero went north. This is room 2.");
    }

    // Test the travel_interpreter function with an invalid command.
    #[test]
    fn travel_interpreter_invalid_command_test() {
        let mut game_state = state::GameState::new();
        let command = ret_lang::parse_input("endure").unwrap_or_else(|e| panic!("{}", e));
        let output = travel_interpreter(&command, &mut game_state);
        assert_eq!(output, Err("Not able to do that action right now."));
    }
}
