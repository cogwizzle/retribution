//! # Command
//!
//! The command module contains all of the structs and enums that are used to parse the input from the user.

use super::*;

macro_rules! create_command {
    (
        $(#[$doc:meta])*
        $name:ident, $target:ty
    ) => {
        $(#[$doc])*
        #[derive(Debug)]
        pub struct $name {
            pub name: String,
            pub description: String,
            pub target: $target
        }
    }
}

create_command!(
    /// A struct that holds the name, description, and target of an AidCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    AidCommand,
    String
);

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
    /// let aid = AidCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(aid.name, "aid");
    /// assert_eq!(aid.description, "Aid an ally in a fight.");
    /// assert_eq!(aid.target, "ally");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<AidCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for aid command.");
        }
        let name = sentence[0];
        Ok(AidCommand {
            name: String::from(name),
            description: String::from("Aid an ally in a fight."),
            target: String::from(sentence[1])
        })
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
    /// let cast = CastCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(cast.name, "cast");
    /// assert_eq!(cast.description, "Cast a spell.");
    /// assert_eq!(cast.spell_name, "fireball");
    /// assert_eq!(cast.target, Some(String::from("goblin")));
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<CastCommand, &str> {
        if sentence.len() < 3 {
            return Err("Not enough arguments for cast command.");
        }
        Ok(CastCommand {
            name: String::from(CAST),
            description: String::from("Cast a spell."),
            spell_name: String::from(sentence[1]),
            target: match sentence.len() {
                0..=2 => None,
                _ => Some(String::from(sentence[2]))
            }
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a DefendCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    DefendCommand,
    String
);

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
    /// let defend = DefendCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(defend.name, "defend");
    /// assert_eq!(defend.description, "Defend an ally in a fight.");
    /// assert_eq!(defend.target, "ally");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<DefendCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for cast command.");
        }
        Ok(DefendCommand {
            name: String::from(sentence[0]),
            description: String::from("Defend an ally in a fight."),
            target: String::from(sentence[1])
        })
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
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::DefyDangerCommand;
    ///
    /// let sentence = vec!["defy", "wizard"];
    /// let defy = DefyDangerCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    ///
    /// assert_eq!(defy.name, "defy");
    /// assert_eq!(defy.description, "Defy danger using a stat.");
    /// assert_eq!(defy.target, Some(String::from("wizard")));
    /// assert_eq!(defy.stat, "wisdom");
    ///
    /// let sentence = vec!["dodge"];
    /// let dodge = DefyDangerCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    ///
    /// assert_eq!(dodge.name, "dodge");
    /// assert_eq!(dodge.description, "Defy danger using a stat.");
    /// assert_eq!(dodge.target, None);
    /// assert_eq!(dodge.stat, "dexterity");
    ///
    /// let sentence = vec!["dodge", "goblin"];
    /// let dodge = DefyDangerCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    ///
    /// assert_eq!(dodge.name, "dodge");
    /// assert_eq!(dodge.description, "Defy danger using a stat.");
    /// assert_eq!(dodge.target, Some(String::from("goblin")));
    /// assert_eq!(dodge.stat, "dexterity");
    ///
    /// let sentence = vec!["charm", "goblin"];
    /// let charm = DefyDangerCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    ///
    /// assert_eq!(charm.name, "charm");
    /// assert_eq!(charm.description, "Defy danger using a stat.");
    /// assert_eq!(charm.target, Some(String::from("goblin")));
    /// assert_eq!(charm.stat, "charisma");
    ///
    /// let sentence = vec!["endure"];
    /// let endure = DefyDangerCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    ///
    /// assert_eq!(endure.name, "endure");
    /// assert_eq!(endure.description, "Defy danger using a stat.");
    /// assert_eq!(endure.target, None);
    /// assert_eq!(endure.stat, "constitution");
    ///
    /// let sentence = vec!["improvise"];
    /// let improvise = DefyDangerCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    ///
    /// assert_eq!(improvise.name, "improvise");
    /// assert_eq!(improvise.description, "Defy danger using a stat.");
    /// assert_eq!(improvise.target, None);
    /// assert_eq!(improvise.stat, "intelligence");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<DefyDangerCommand, &str> {
        if sentence.len() < 1 {
            return Err("Not enough arguments for defy danger command.");
        }
        let name = sentence[0];
        Ok(DefyDangerCommand {
            name: String::from(name),
            description: String::from("Defy danger using a stat."),
            target: match sentence.len() {
                1 => None,
                _ => Some(String::from(sentence[1]))
            },
            stat: match name {
                CHARM => String::from("charisma"),
                DEFY => String::from("wisdom"),
                DODGE => String::from("dexterity"),
                ENDURE => String::from("constitution"),
                IMPROVISE => String::from("intelligence"),
                _ => String::from("dexterity") 
            }
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a DiscernRealitiesCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - An optional string that holds the target of the command.
    DiscernRealitiesCommand,
    Option<String>
);

impl DiscernRealitiesCommand {
    /// Construct new DiscernRealitiesCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::DiscernRealitiesCommand;
    ///
    /// let sentence = vec!["search"];
    /// let search = DiscernRealitiesCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(search.name, "search");
    /// assert_eq!(search.description, "Discern realities about a subject.");
    /// assert_eq!(search.target, None);
    ///
    /// let sentence = vec!["study", "goblin"];
    /// let search = DiscernRealitiesCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(search.name, "study");
    /// assert_eq!(search.description, "Discern realities about a subject.");
    /// assert_eq!(search.target, Some(String::from("goblin")));
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<DiscernRealitiesCommand, &str> {
        if sentence.len() < 1 {
            return Err("Not enough arguments for discern realities command.");
        }
        Ok(DiscernRealitiesCommand {
            name: String::from(sentence[0]),
            description: String::from("Discern realities about a subject."),
            target: match sentence.len() {
                0..=1 => None,
                _ => Some(String::from(sentence[1]))
            }
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a DropCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    DropCommand,
    String
);

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
    /// let drop = DropCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(drop.name, "drop");
    /// assert_eq!(drop.description, "Drops an item from the player's inventory.");
    /// assert_eq!(drop.target, "sword");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<DropCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for drop command.");
        }
        Ok(DropCommand {
            name: String::from(DROP),
            description: String::from("Drops an item from the player's inventory."),
            target: String::from(sentence[1])
        })
    }
}

/// A struct that holds the name, description, and target of an EndureHarmCommand.
///
/// # Attributes
/// * `name` - A string that holds the name of the command.
/// * `description` - A string that holds the description of the command.
pub struct ExitCommand {
    pub name: String,
    pub description: String
}

impl ExitCommand {
    /// Construct new ExitCommand.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::ExitCommand;
    ///
    /// let exit = ExitCommand::build().unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(exit.name, "exit");
    /// assert_eq!(exit.description, "Exits the game.");
    /// ```
    pub fn build<'a>() -> Result<ExitCommand, &'a str> {
        Ok(ExitCommand {
            name: String::from(EXIT),
            description: String::from("Exits the game.")
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a GoCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    GoCommand,
    String
);

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
    /// let go = GoCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(go.name, "go");
    /// assert_eq!(go.description, "Moves the player to a new location.");
    /// assert_eq!(go.target, "north");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<GoCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for go command.");
        }
        Ok(GoCommand {
            name: String::from(GO),
            description: String::from("Moves the player to a new location."),
            target: String::from(sentence[1])
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a HackAndSlashCommand.
    /// 
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - An optional string that holds the target of the command.
    HackAndSlashCommand,
    Vec<String>
);

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
    /// let hack = HackAndSlashCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(hack.name, "attack");
    /// assert_eq!(hack.description, "Attack an enemy with a melee weapon.");
    /// assert_eq!(hack.target, vec!["goblin"]);
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<HackAndSlashCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for hack and slash command.");
        }
        let name = *sentence.first().unwrap_or_else(|| panic!("No command found."));
        Ok(HackAndSlashCommand {
            name: String::from(name),
            description: String::from("Attack an enemy with a melee weapon."),
            target: sentence[1..].iter().map(|s| String::from(*s)).collect()
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a HelpCommand.
    ///     
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - An optional string that holds the target of the command.
    HelpCommand,
    Option<String>
);

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
    /// let help = HelpCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(help.name, "help");
    /// assert_eq!(help.description, "Prints a list of commands or the description of a command.");
    /// assert_eq!(help.target, None);
    /// ```
    ///
    /// ```
    /// use retribution::ret_lang::HelpCommand;
    ///
    /// let sentence = vec!["help", "go"];
    /// let help = HelpCommand::build(sentence);
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<HelpCommand, &str> {
        if sentence.len() < 1 {
            return Err("Not enough arguments for help command.");
        }
        Ok(HelpCommand {
            name: String::from(HELP),
            description: String::from("Prints a list of commands or the description of a command."),
            target: match sentence.len() {
                1 => None,
                _ => Some(String::from(sentence[1]))
            }
        })
    }
}

create_command!(
    /// A struct that holds the name, description and target of an InterfereCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    InterfereCommand,
    String
);

impl InterfereCommand {
    /// Construct new InterfereCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::InterfereCommand;
    ///
    /// let sentence = vec!["interfere", "goblin"];
    /// let interfere = InterfereCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(interfere.name, "interfere");
    /// assert_eq!(interfere.description, "Interfere with an enemy's attack.");
    /// assert_eq!(interfere.target, "goblin");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<InterfereCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for interfere command.");
        }
        Ok(InterfereCommand {
            name: String::from(INTERFERE),
            description: String::from("Interfere with an enemy's attack."),
            target: String::from(sentence[1])
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a ParleyCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    ParleyCommand,
    String
);

impl ParleyCommand {
    /// Construct new ParleyCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::ParleyCommand;
    ///
    /// let sentence = vec!["parley", "goblin"];
    /// let parley = ParleyCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(parley.name, "parley");
    /// assert_eq!(parley.description, "Parley with an enemy.");
    /// assert_eq!(parley.target, "goblin");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<ParleyCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for parley command.");
        }
        Ok(ParleyCommand {
            name: String::from(PARLEY),
            description: String::from("Parley with an enemy."),
            target: String::from(sentence[1])
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a SayCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the value of the command.
    SayCommand,
    String
);

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
    /// let say = SayCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(say.name, "say");
    /// assert_eq!(say.description, "Prints a message to the screen.");
    /// assert_eq!(say.target, "hello world");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<SayCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for say command.");
        }
        Ok(SayCommand {
            name: String::from(SAY),
            description: String::from("Prints a message to the screen."),
            target: sentence[1..].join(" ")
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a SpoutLoreCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - An optional string that holds the target of the command.
    SpoutLoreCommand,
    Option<String>
);

impl SpoutLoreCommand {
    /// Construct new SpoutLoreCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    ///
    /// # Examples
    /// ```
    /// use retribution::ret_lang::SpoutLoreCommand;
    ///
    /// let sentence = vec!["consult", "wizard"];
    /// let spout = SpoutLoreCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(spout.name, "consult");
    /// assert_eq!(spout.description, "Spout lore about a subject.");
    /// assert_eq!(spout.target, Some(String::from("wizard")));
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<SpoutLoreCommand, &str> {
        if sentence.len() < 1 {
            return Err("Not enough arguments for spout lore command.");
        }
        Ok(SpoutLoreCommand {
            name: String::from(sentence[0]),
            description: String::from("Spout lore about a subject."),
            target: match sentence.len() {
                0..=1 => None,
                _ => Some(String::from(sentence[1]))
            }
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a TakeCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    TakeCommand,
    String
);

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
    /// let take = TakeCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(take.name, "take");
    /// assert_eq!(take.description, "Takes an item from the current location.");
    /// assert_eq!(take.target, "sword");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<TakeCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for take command.");
        }
        Ok(TakeCommand {
            name: String::from(TAKE),
            description: String::from("Takes an item from the current location."),
            target: String::from(sentence[1])
        })
    }
}

create_command!(
    /// A struct that holds the name, description, and target of a VolleyCommand.
    ///
    /// # Attributes
    /// * `name` - A string that holds the name of the command.
    /// * `description` - A string that holds the description of the command.
    /// * `target` - A string that holds the target of the command.
    VolleyCommand,
    String
);

impl VolleyCommand {
    /// Construct new VolleyCommand.
    ///
    /// # Arguments
    /// * `sentence` - A vector of string slices that holds the line of text to tokenize.
    /// 
    /// # Examples
    /// ```
    /// use retribution::ret_lang::VolleyCommand;
    ///
    /// let sentence = vec!["volley", "goblin"];
    /// let volley = VolleyCommand::build(sentence).unwrap_or_else(|e| panic!("{}", e));
    /// assert_eq!(volley.name, "volley");
    /// assert_eq!(volley.description, "Attack an enemy with a ranged weapon.");
    /// assert_eq!(volley.target, "goblin");
    /// ```
    pub fn build(sentence: Vec<&str>) -> Result<VolleyCommand, &str> {
        if sentence.len() < 2 {
            return Err("Not enough arguments for volley command.");
        }
        Ok(VolleyCommand {
            name: String::from(sentence[0]),
            description: String::from("Attack an enemy with a ranged weapon."),
            target: String::from(sentence[1])
        })
    }
}

/// An enum that holds all of the possible commands.
pub enum Command {
    Aid(AidCommand),
    Cast(CastCommand),
    Defend(DefendCommand),
    DefyDanger(DefyDangerCommand),
    DiscernRealities(DiscernRealitiesCommand),
    Drop(DropCommand),
    Exit(ExitCommand),
    Go(GoCommand),
    HackAndSlash(HackAndSlashCommand),
    Help(HelpCommand),
    Interfere(InterfereCommand),
    Parley(ParleyCommand),
    Say(SayCommand),
    SpoutLore(SpoutLoreCommand),
    Take(TakeCommand),
    Volley(VolleyCommand),
}
