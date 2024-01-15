use retribution::game;
use retribution::game::interpreter;
use retribution::game::map;
use retribution::game::state;
use retribution::plugin;
use retribution::ret_lang;
use std::io;

fn main() {
    game::init().unwrap();
    let test_map = map::load_map("Test Area", None).unwrap();
    let mut game_state = state::GameState::new();
    game_state.map = Some(test_map);
    game_state.room = Some((1, 1));
    let mut reader = io::stdin();
    let state_writer = plugin::StateWriter::new(None);
    // We don't care if the state writer fails as the game will continue
    // to function as normal.
    let _ = state_writer.write_state(game_state.clone()).map_err(|_| ());

    // Main game loop.
    loop {
        let input = match game::prompt(&mut reader) {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let command = match ret_lang::parse_input(&input[..]) {
            Ok(c) => c,
            _ => {
                println!("{} is not a valid command.", input.trim());
                continue;
            }
        };
        let output = interpreter::interpreter(&command, &mut game_state);
        match output {
            Ok(o) => println!("{}", o),
            Err(e) => println!("{}", e),
        }
        // We don't care if the state writer fails as the game will continue
        // to function as normal.
        let _ = state_writer.write_state(game_state.clone()).map_err(|_| ());
    }
}
