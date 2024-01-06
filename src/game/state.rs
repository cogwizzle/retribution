/// A module that contains the state of the game.
pub struct GameState {
    pub mode: Mode,
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
        }
    }
}

pub enum Mode {
    Combat,
    Menu,
    Travel,
}
