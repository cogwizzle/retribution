//! # Parser
//! The parser module contains functions for parsing user input into commands.

use crate::ret_lang::command::*;
use super::*;

/// Tokenize a line of text into a vector of words.
///
/// # Arguments
/// * `line` - A string slice that holds the line of text to tokenize.
fn tokenize(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

/// Parse a line of text and return the command definition.
/// # Arguments
/// * `line` - A string slice that holds the line of text to parse.
///
/// # Examples
/// ```
/// use retribution::ret_lang::parse_input;
///
/// let sentence = "say hello world";
/// parse_input(sentence);
/// ```
pub fn parse_input(line: &str) -> Result<Command, &str> {
    let tokens = tokenize(line);
    let command = tokens[0];
    match command {
        AID | ASSIST => {
            let command = AidCommand::build(tokens)?;
            Ok(Command::Aid(command))
        },
        ATTACK | FIGHT | HIT => {
            let command = HackAndSlashCommand::build(tokens)?;
            Ok(Command::HackAndSlash(command))
        },
        CAST => {
            let command = CastCommand::build(tokens)?;
            Ok(Command::Cast(command))
        },
        CONSULT => {
            let command = SpoutLoreCommand::build(tokens)?;
            Ok(Command::SpoutLore(command))
        },
        CHARM | DEFY | DODGE | ENDURE | IMPROVISE => {
            let command = DefyDangerCommand::build(tokens)?;
            Ok(Command::DefyDanger(command))
        },
        DEFEND | PROTECT => {
            let command = DefendCommand::build(tokens)?;
            Ok(Command::Defend(command))
        },
        DROP => {
            let command = DropCommand::build(tokens)?;
            Ok(Command::Drop(command))
        },
        GO => {
            let command = GoCommand::build(tokens)?;
            Ok(Command::Go(command))
        },
        HELP => {
            let command = HelpCommand::build(tokens)?;
            Ok(Command::Help(command))
        },
        INTERFERE => {
            let command = InterfereCommand::build(tokens)?;
            Ok(Command::Interfere(command))
        },
        PARLEY => {
            let command = ParleyCommand::build(tokens)?;
            Ok(Command::Parley(command))
        },
        SAY => {
            let command = SayCommand::build(tokens)?;
            Ok(Command::Say(command))
        },
        SEARCH | STUDY => {
            let command = DiscernRealitiesCommand::build(tokens)?;
            Ok(Command::DiscernRealities(command))
        },
        SHOOT | VOLLEY => {
            let command = VolleyCommand::build(tokens)?;
            Ok(Command::Volley(command))
        },
        TAKE => {
            let command = TakeCommand::build(tokens)?;
            Ok(Command::Take(command))
        },
        _ => Err("Command not found."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the tokenize function.
    #[test]
    fn test_tokenize() {
        let sentence = "say hello world";
        let tokens = tokenize(sentence);
        assert_eq!(tokens, vec!["say", "hello", "world"]);
    }

    /// Test the parse_input function with an aid command.
    #[test]
    fn test_parse_aid() {
        let sentence = "aid ally";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Aid(aid) => {
                assert_eq!(aid.name, "aid");
                assert_eq!(aid.description, "Aid an ally in a fight.");
                assert_eq!(aid.target, "ally");
            },
            _ => panic!("Aid command expected."),
        }
    }

    /// Test the parse_input function with an attack command.
    #[test]
    fn test_parse_attack() {
        let sentence = "attack goblin";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::HackAndSlash(hack) => {
                assert_eq!(hack.name, "attack");
                assert_eq!(hack.description, "Attack an enemy with a melee weapon.");
                assert_eq!(hack.target, vec!["goblin"]);
            },
            _ => panic!("Attack command expected."),
        }
    }

    /// Test the parse_input for other hack and slash command.
    #[test]
    fn test_parse_hack_and_slash() {
        let sentence = "fight goblin";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::HackAndSlash(hack) => {
                assert_eq!(hack.name, "fight");
                assert_eq!(hack.description, "Attack an enemy with a melee weapon.");
                assert_eq!(hack.target, vec!["goblin"]);
            },
            _ => panic!("Hack and slash command expected."),
        }
    }

    /// Test the parse_input function with a cast command.
    #[test]
    fn test_parse_cast() {
        let sentence = "cast fireball goblin";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Cast(cast) => {
                assert_eq!(cast.name, "cast");
                assert_eq!(cast.description, "Cast a spell.");
                assert_eq!(cast.spell_name, "fireball");
                assert_eq!(cast.target, Some(String::from("goblin")));
            },
            _ => panic!("Cast command expected."),
        }
    }

    /// Test the parse_input function witha  defend command.
    #[test]
    fn test_parse_defend() {
        let sentence = "protect ally";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Defend(defend) => {
                assert_eq!(defend.name, "protect");
                assert_eq!(defend.description, "Defend an ally in a fight.");
                assert_eq!(defend.target, "ally");
            },
            _ => panic!("Defend command expected."),
        }
    }

    /// Test the parse_input function with a defy danger command.
    #[test]
    fn test_parse_defy_danger() {
        let sentence = "dodge";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::DefyDanger(defy) => {
                assert_eq!(defy.name, "dodge");
                assert_eq!(defy.description, "Defy danger using a stat.");
                assert_eq!(defy.target, None);
                assert_eq!(defy.stat, "dexterity");
            },
            _ => panic!("Defy danger command expected."),
        }
    }

    /// Test the parse_input function with a discern realities command.
    #[test]
    fn test_parse_discern_realities() {
        let sentence = "search";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::DiscernRealities(discern) => {
                assert_eq!(discern.name, "search");
                assert_eq!(discern.description, "Discern realities about a subject.");
                assert_eq!(discern.target, None);
            },
            _ => panic!("Discern realities command expected."),
        }
    }

    /// Test the parse_input function with a drop command.
    #[test]
    fn test_parse_drop() {
        let sentence = "drop sword";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Drop(drop) => {
                assert_eq!(drop.name, "drop");
                assert_eq!(drop.description, "Drops an item from the player's inventory.");
                assert_eq!(drop.target, "sword");
            },
            _ => panic!("Drop command expected."),
        }
    }

    /// Test the parse_input function with a go command.
    #[test]
    fn test_parse_go() {
        let sentence = "go north";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Go(go) => {
                assert_eq!(go.name, "go");
                assert_eq!(go.description, "Moves the player to a new location.");
                assert_eq!(go.target, "north");
            },
            _ => panic!("Go command expected."),
        }
    }

    /// Test the parse_input function with a help command.
    #[test]
    fn test_parse_help() {
        let sentence = "help";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Help(help) => {
                assert_eq!(help.name, "help");
                assert_eq!(help.description, "Prints a list of commands or the description of a command.");
                assert_eq!(help.target, None);
            },
            _ => panic!("Help command expected."),
        }
    }

    /// Test the parse_input function with a help command with a target.
    #[test]
    fn test_parse_help_target() {
        let sentence = "help go";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Help(help) => {
                assert_eq!(help.name, "help");
                assert_eq!(help.description, "Prints a list of commands or the description of a command.");
                assert_eq!(help.target, Some(String::from("go")));
            },
            _ => panic!("Help command expected."),
        }
    }

    /// Test the parse_input function with an interfere command.
    #[test]
    fn test_parse_interfere() {
        let sentence = "interfere goblin";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Interfere(interfere) => {
                assert_eq!(interfere.name, "interfere");
                assert_eq!(interfere.description, "Interfere with an enemy's attack.");
                assert_eq!(interfere.target, "goblin");
            },
            _ => panic!("Interfere command expected."),
        }
    }

    /// Test the parse_input function with a parley command.
    #[test]
    fn test_parse_parley() {
        let sentence = "parley goblin";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Parley(parley) => {
                assert_eq!(parley.name, "parley");
                assert_eq!(parley.description, "Parley with an enemy.");
                assert_eq!(parley.target, "goblin");
            },
            _ => panic!("Parley command expected."),
        }
    }

    /// Test the parse_input function with a say command.
    #[test]
    fn test_parse_say() {
        let sentence = "say hello world";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Say(say) => {
                assert_eq!(say.name, "say");
                assert_eq!(say.description, "Prints a message to the screen.");
                assert_eq!(say.target, "hello world");
            },
            _ => panic!("Say command expected."),
        }
    }

    /// Test the parse_input function with a take command.
    #[test]
    fn test_parse_take() {
        let sentence = "take sword";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Take(take) => {
                assert_eq!(take.name, "take");
                assert_eq!(take.description, "Takes an item from the current location.");
                assert_eq!(take.target, "sword");
            },
            _ => panic!("Take command expected."),
        }
    }

    /// Test the parse_input function with a volley command.
    #[test]
    fn test_parse_volley() {
        let sentence = "shoot goblin";
        let comamnd = parse_input(sentence).unwrap_or_else(|e| panic!("{}", e));
        match comamnd {
            Command::Volley(volley) => {
                assert_eq!(volley.name, "shoot");
                assert_eq!(volley.description, "Attack an enemy with a ranged weapon.");
                assert_eq!(volley.target, "goblin");
            },
            _ => panic!("Volley command expected."),
        }
    }
}
