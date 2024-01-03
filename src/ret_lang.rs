//! A module that holds the language for the Retribution prompt language.
//! This module should only contain information about the language itself,
//! and not implementation details about the game.

const AID: &str = "aid";
const ASSIST: &str = "assist";
const ATTACK: &str = "attack";
const CAST: &str = "cast";
const CHARM: &str = "charm";
const CONSULT: &str = "consult";
const DEFEND: &str = "defend";
const DODGE: &str = "dodge";
const DROP: &str = "drop";
const ENDURE: &str = "endure";
const FIGHT: &str = "fight";
const GO: &str = "go";
const HELP: &str = "help";
const HIT: &str = "hit";
const INTERFERE: &str = "interfere";
const IMPROVISE: &str = "improvise";
const PARLEY: &str = "parley";
const PROTECT: &str = "protect";
const SAY: &str = "say";
const SEARCH: &str = "search";
const SHOOT: &str = "shoot";
const STUDY: &str = "study";
const TAKE: &str = "take";
const VOLLEY: &str = "volley";

/// Tokenize a line of text into a vector of words.
///
/// # Arguments
/// * `line` - A string slice that holds the line of text to tokenize.
fn tokenize(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

/// A struct that holds the name, description, and target of a AidCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - An optional string that holds the target of the command.
#[derive(Debug)]
pub struct AidCommand {
    pub name: String,
    pub description: String,
    pub target: String
}

impl AidCommand {
    /// Construct new AidCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::AidCommand;
    ///
    /// let sentence = vec!["aid", "ally"];
    /// let aid = AidCommand::new(sentence);
    /// assert_eq!(aid.name, "aid");
    /// assert_eq!(aid.description, "Aid an ally in a fight.");
    /// assert_eq!(aid.target, "ally");
    /// ```
    pub fn new(sentence: Vec<&str>) -> AidCommand {
        let name = *sentence.first().unwrap_or_else(|| panic!("No command found."));
        AidCommand {
            name: String::from(name),
            description: String::from("Aid an ally in a fight."),
            target: String::from(sentence[1])
        }
    }
}

/// A struct that holds the name, description, and target of a CastCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `spell_name` - A string that holds the name of the spell to cast.
/// * `target` - An optional string that holds the target of the command.
#[derive(Debug)]
pub struct CastCommand {
    pub name: String,
    pub description: String,
    pub spell_name: String,
    pub target: Option<String>
}

impl CastCommand {
    /// Construct new CastCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::CastCommand;
    ///
    /// let sentence = vec!["cast", "fireball", "goblin"];
    /// let cast = CastCommand::new(sentence);
    /// assert_eq!(cast.name, "cast");
    /// assert_eq!(cast.description, "Cast a spell.");
    /// assert_eq!(cast.spell_name, "fireball");
    /// assert_eq!(cast.target, Some(String::from("goblin")));
    /// ```
    pub fn new(sentence: Vec<&str>) -> CastCommand {
        CastCommand {
            name: String::from(CAST),
            description: String::from("Cast a spell."),
            spell_name: String::from(sentence[1]),
            target: match sentence.len() {
                0..=2 => None,
                _ => Some(String::from(sentence[2]))
            }
        }
    }
}

/// A struct that holds the name, description, and target of a DefendCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - A string that holds the target of the command.
#[derive(Debug)]
pub struct DefendCommand {
    pub name: String,
    pub description: String,
    pub target: String
}

impl DefendCommand {
    /// Construct new DefendCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::DefendCommand;
    ///
    /// let sentence = vec!["defend", "ally"];
    /// let defend = DefendCommand::new(sentence);
    /// assert_eq!(defend.name, "defend");
    /// assert_eq!(defend.description, "Defend an ally in a fight.");
    /// assert_eq!(defend.target, "ally");
    /// ```
    pub fn new(sentence: Vec<&str>) -> DefendCommand {
        DefendCommand {
            name: String::from(sentence[0]),
            description: String::from("Defend an ally in a fight."),
            target: String::from(sentence[1])
        }
    }
}

/// A struct that holds the name, description, and target of a DefyDangerCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - An optional string that holds the target of the command.
/// * `stat` - A string that holds the stat to use for the defy danger roll.
#[derive(Debug)]
pub struct DefyDangerCommand {
    pub name: String,
    pub description: String,
    pub target: Option<String>,
    pub stat: String
}

impl DefyDangerCommand {
    /// Construct new DefyDangerCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    /// * `stat` - A string that holds the stat to use for the defy danger roll.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::DefyDangerCommand;
    ///
    /// let sentence = vec!["dodge"];
    /// let dodge = DefyDangerCommand::new(sentence);
    ///
    /// assert_eq!(dodge.name, "dodge");
    /// assert_eq!(dodge.description, "Defy danger using a stat.");
    /// assert_eq!(dodge.target, None);
    /// assert_eq!(dodge.stat, "dexterity");
    ///
    /// let sentence = vec!["dodge", "goblin"];
    /// let dodge = DefyDangerCommand::new(sentence);
    ///
    /// assert_eq!(dodge.name, "dodge");
    /// assert_eq!(dodge.description, "Defy danger using a stat.");
    /// assert_eq!(dodge.target, Some(String::from("goblin")));
    /// assert_eq!(dodge.stat, "dexterity");
    ///
    /// let sentence = vec!["charm", "goblin"];
    /// let charm = DefyDangerCommand::new(sentence);
    ///
    /// assert_eq!(charm.name, "charm");
    /// assert_eq!(charm.description, "Defy danger using a stat.");
    /// assert_eq!(charm.target, Some(String::from("goblin")));
    /// assert_eq!(charm.stat, "charisma");
    ///
    /// let sentence = vec!["endure"];
    /// let endure = DefyDangerCommand::new(sentence);
    ///
    /// assert_eq!(endure.name, "endure");
    /// assert_eq!(endure.description, "Defy danger using a stat.");
    /// assert_eq!(endure.target, None);
    /// assert_eq!(endure.stat, "constitution");
    ///
    /// let sentence = vec!["improvise"];
    /// let improvise = DefyDangerCommand::new(sentence);
    ///
    /// assert_eq!(improvise.name, "improvise");
    /// assert_eq!(improvise.description, "Defy danger using a stat.");
    /// assert_eq!(improvise.target, None);
    /// assert_eq!(improvise.stat, "intelligence");
    /// ```
    pub fn new(sentence: Vec<&str>) -> DefyDangerCommand {
        let name = sentence[0];
        DefyDangerCommand {
            name: String::from(name),
            description: String::from("Defy danger using a stat."),
            target: match sentence.len() {
                1 => None,
                _ => Some(String::from(sentence[1]))
            },
            stat: match name {
                CHARM => String::from("charisma"),
                ENDURE => String::from("constitution"),
                IMPROVISE => String::from("intelligence"),
                _ => String::from("dexterity") 
            }
        }
    }
}

/// A struct that holds the name, description, and target of a DropCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - A string that holds the target of the command.
#[derive(Debug)]
pub struct DropCommand {
    pub name: String,
    pub description: String,
    pub target: String
}

impl DropCommand {
    /// Construct new DropCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::DropCommand;
    ///
    /// let sentence = vec!["drop", "sword"];
    /// let drop = DropCommand::new(sentence);
    /// assert_eq!(drop.name, "drop");
    /// assert_eq!(drop.description, "Drops an item from the player's inventory.");
    /// assert_eq!(drop.target, "sword");
    /// ```
    pub fn new(sentence: Vec<&str>) -> DropCommand {
        DropCommand {
            name: String::from(DROP),
            description: String::from("Drops an item from the player's inventory."),
            target: String::from(sentence[1])
        }
    }
}

/// A struct that holds the name, description, and target of a GoCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - A string that holds the target of the command.
#[derive(Debug)]
pub struct GoCommand {
    pub name: String,
    pub description: String,
    pub target: String
}

impl GoCommand {
    /// Construct new GoCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::GoCommand;
    ///
    /// let sentence = vec!["go", "north"];
    /// let go = GoCommand::new(sentence);
    /// assert_eq!(go.name, "go");
    /// assert_eq!(go.description, "Moves the player to a new location.");
    /// assert_eq!(go.target, "north");
    /// ```
    pub fn new(sentence: Vec<&str>) -> GoCommand {
        GoCommand {
            name: String::from(GO),
            description: String::from("Moves the player to a new location."),
            target: String::from(sentence[1])
        }
    }
}

/// A struct taht holds the name, description, and target of a HackAndSlashCommand.
/// 
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - An optional string that holds the target of the command.

#[derive(Debug)]
pub struct HackAndSlashCommand {
    pub name: String,
    pub description: String,
    pub target: Vec<String>
}

impl HackAndSlashCommand {
    /// Construct new HackAndSlashCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::HackAndSlashCommand;
    ///
    /// let sentence = vec!["attack", "goblin"];
    /// let hack = HackAndSlashCommand::new(sentence);
    /// assert_eq!(hack.name, "attack");
    /// assert_eq!(hack.description, "Attack an enemy with a melee weapon.");
    /// assert_eq!(hack.target, vec!["goblin"]);
    /// ```
    pub fn new(sentence: Vec<&str>) -> HackAndSlashCommand {
        let name = *sentence.first().unwrap_or_else(|| panic!("No command found."));
        HackAndSlashCommand {
            name: String::from(name),
            description: String::from("Attack an enemy with a melee weapon."),
            target: sentence[1..].iter().map(|s| String::from(*s)).collect()
        }
    }
}

/// A struct that holds the name, description, and target of a HelpCommand.
///     
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - An optional string that holds the target of the command.
#[derive(Debug)]
pub struct HelpCommand {
    pub name: String,
    pub description: String,
    pub target: Option<String>
}

impl HelpCommand {
    /// Construct new HelpCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::HelpCommand;
    ///
    /// let sentence = vec!["help"];
    /// let help = HelpCommand::new(sentence);
    /// assert_eq!(help.name, "help");
    /// assert_eq!(help.description, "Prints a list of commands or the description of a command.");
    /// assert_eq!(help.target, None);
    /// ```
    ///
    /// ```
    /// use retribution::ret_lang::HelpCommand;
    ///
    /// let sentence = vec!["help", "go"];
    /// let help = HelpCommand::new(sentence);
    /// ```
    pub fn new(sentence: Vec<&str>) -> HelpCommand {
        HelpCommand {
            name: String::from(HELP),
            description: String::from("Prints a list of commands or the description of a command."),
            target: match sentence.len() {
                1 => None,
                _ => Some(String::from(sentence[1]))
            }
        }
    }
}

/// A struct that holds the name, description, and target of a SayCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `value` - A string that holds the value of the command.
#[derive(Debug)]
pub struct SayCommand {
    pub name: String,
    pub description: String,
    pub target: String
}

impl SayCommand {
    /// Construct new SayCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::SayCommand;
    ///
    /// let sentence = vec!["say", "hello", "world"];
    /// let say = SayCommand::new(sentence);
    /// assert_eq!(say.name, "say");
    /// assert_eq!(say.description, "Prints a message to the screen.");
    /// assert_eq!(say.target, "hello world");
    /// ```
    pub fn new(sentence: Vec<&str>) -> SayCommand {
        SayCommand {
            name: String::from(SAY),
            description: String::from("Prints a message to the screen."),
            target: sentence[1..].join(" ")
        }
    }
}

/// A struct that holds the name, description, and target of a TakeCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
/// * `target` - A string that holds the target of the command.
#[derive(Debug)]
pub struct TakeCommand {
    pub name: String,
    pub description: String,
    pub target: String
}

impl TakeCommand {
    /// Construct new TakeCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::TakeCommand;
    ///
    /// let sentence = vec!["take", "sword"];
    /// let take = TakeCommand::new(sentence);
    /// assert_eq!(take.name, "take");
    /// assert_eq!(take.description, "Takes an item from the current location.");
    /// assert_eq!(take.target, "sword");
    /// ```
    pub fn new(sentence: Vec<&str>) -> TakeCommand {
        TakeCommand {
            name: String::from(TAKE),
            description: String::from("Takes an item from the current location."),
            target: String::from(sentence[1])
        }
    }
}

/// An enum that holds all of the possible commands.
pub enum Command {
    Aid(AidCommand),
    Cast(CastCommand),
    Defend(DefendCommand),
    DefyDanger(DefyDangerCommand),
    Drop(DropCommand),
    HackAndSlash(HackAndSlashCommand),
    Help(HelpCommand),
    Go(GoCommand),
    Say(SayCommand),
    Take(TakeCommand),
}

/// Parse a line of text and execute the command.
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
/// # Panics
/// Panics if the command is not found.
pub fn parse_input(line: &str) -> Command {
    let tokens = tokenize(line);
    let command = tokens[0];
    match command {
        AID | ASSIST => Command::Aid(AidCommand::new(tokens)),
        ATTACK | FIGHT | HIT => Command::HackAndSlash(HackAndSlashCommand::new(tokens)),
        CAST => Command::Cast(CastCommand::new(tokens)),
        CHARM | DODGE | ENDURE | IMPROVISE => Command::DefyDanger(DefyDangerCommand::new(tokens)),
        DEFEND | PROTECT => Command::Defend(DefendCommand::new(tokens)),
        DROP => Command::Drop(DropCommand::new(tokens)),
        GO => Command::Go(GoCommand::new(tokens)),
        HELP => Command::Help(HelpCommand::new(tokens)),
        SAY => Command::Say(SayCommand::new(tokens)),
        TAKE => Command::Take(TakeCommand::new(tokens)),
        _ => panic!("Command not found."),
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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

    /// Test the parse_input function with a drop command.
    #[test]
    fn test_parse_drop() {
        let sentence = "drop sword";
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
        match comamnd {
            Command::Help(help) => {
                assert_eq!(help.name, "help");
                assert_eq!(help.description, "Prints a list of commands or the description of a command.");
                assert_eq!(help.target, Some(String::from("go")));
            },
            _ => panic!("Help command expected."),
        }
    }

    /// Test the parse_input function with a say command.
    #[test]
    fn test_parse_say() {
        let sentence = "say hello world";
        let comamnd = parse_input(sentence);
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
        let comamnd = parse_input(sentence);
        match comamnd {
            Command::Take(take) => {
                assert_eq!(take.name, "take");
                assert_eq!(take.description, "Takes an item from the current location.");
                assert_eq!(take.target, "sword");
            },
            _ => panic!("Take command expected."),
        }
    }
}
