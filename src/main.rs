use retribution::game;
use retribution::game::interpreter;
use retribution::ret_lang;

fn main() {
    loop {
        let input = match game::prompt() {
            Ok(i) => i,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        if input == "exit\n" {
            break;
        }
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
