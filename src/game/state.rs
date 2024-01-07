//! # State
//! This module contains the state of the game.
use crate::game::map;

/// A module that contains the state of the game.
pub struct GameState {
    /// The current mode of the game.
    pub mode: Mode,
    /// The current map the player is in.
    pub map: Option<map::Map>,
    /// The current room the player is in.
    pub room: Option<(usize, usize)>,

}

impl GameState {
    /// A function that creates a new GameState.
    ///
    /// # Returns
    /// * `GameState` - A new GameState.
    ///
    /// # Examples
    /// ```
    /// use retribution::game::state;
    ///
    /// let game_state = state::GameState::new();
    /// ```
    pub fn new() -> GameState {
        GameState {
            mode: Mode::Travel,
            map: None,
            room: None,
        }
    }
}

/// An enum that represents the mode of the game.
pub enum Mode {
    Combat,
    Menu,
    Travel,
}
