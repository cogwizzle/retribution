use retribution::game;
use retribution::game::interpreter;
use retribution::ret_lang;
use std::io;

fn main() {
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
                println!("Not a valid command.");
                continue;
            }
        };
        let output = interpreter::travel_parser(&command);
        match output {
            Ok(o) => println!("{}", o),
            Err(e) => println!("{}", e),
        }
    }
}
