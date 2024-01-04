use retribution::ret_lang;

fn hello() {
    let command = ret_lang::parse_input("say Hello World!").unwrap_or_else(|e| panic!("{}", e));
    match command {
        ret_lang::Command::Say(c) => println!("Hero said: {}", c.target),
        _ => println!("Not a say command"),
    }
}

fn main() {
    hello();
}
