pub struct GameState {
    pub mode: Mode,
}

impl GameState {
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
