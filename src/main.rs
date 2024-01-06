use retribution::game::interpreter;
use retribution::game::state;
use retribution::game;
use retribution::ret_lang;
use std::io;

fn main() {
    let mut game_state = state::GameState::new();

    // Main game loop.
    loop {
        let mut reader = io::stdin();
        let input = match game::prompt(&mut reader) {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let command = ret_lang::parse_input(&input[..]);
        let command = match command {
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
