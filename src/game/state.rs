//! # State
//! This module contains the state of the game.
use crate::game::room;

/// A module that contains the state of the game.
pub struct GameState<'a> {
    /// The current mode of the game.
    pub mode: Mode,
    /// The current room the player is in.
    pub room: Option<&'a room::Room>,
}

impl<'a> GameState<'a> {
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
    pub fn new() -> GameState<'a> {
        GameState {
            mode: Mode::Travel,
            room: None,
        }
    }

    // pub fn set_room(&mut self, room: &'a room::Room) {
    //    self.room = Some(room);
    // }
}

/// An enum that represents the mode of the game.
pub enum Mode {
    Combat,
    Menu,
    Travel,
}
