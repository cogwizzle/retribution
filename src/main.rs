use retribution::game::interpreter;
use retribution::game::state;
use retribution::game;
use retribution::ret_lang;
use std::io;

fn main() {
    let mut game_state = state::GameState::new();
    let mut reader = io::stdin();

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
    }
}
