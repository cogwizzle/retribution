//! A module that holds the language for the Retribution prompt language.

const DROP: &str = "drop";
const GO: &str = "go";
const HELP: &str = "help";
const SAY: &str = "say";
const TAKE: &str = "take";

/// Tokenize a line of text into a vector of words.
///
/// # Arguments
/// * `line` - A string slice that holds the line of text to tokenize.
fn tokenize(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
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
    /// ```
    pub fn new(sentence: Vec<&str>) -> GoCommand {
        GoCommand {
            name: String::from(GO),
            description: String::from("Moves the player to a new location."),
            target: String::from(sentence[1])
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
    Drop(DropCommand),
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
        "drop" => {
            Command::Drop(DropCommand::new(tokens))
        },
        "go" => {
            Command::Go(GoCommand::new(tokens))
        },
        "help" => {
            Command::Help(HelpCommand::new(tokens))
        },
        "say" => {
            Command::Say(SayCommand::new(tokens))
        },
        "take" => {
            Command::Take(TakeCommand::new(tokens))
        },
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
