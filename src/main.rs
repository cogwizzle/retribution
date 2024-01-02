use retribution::ret_lang;

fn hello() {
    let command = ret_lang::parse_input("say Hello World!");
    match command {
        ret_lang::Command::Say(c) => println!("Hero said: {}", c.target),
        _ => println!("Not a say command"),
    }
}

fn main() {
    hello();
}
