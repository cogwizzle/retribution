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
const DEFY: &str = "defy";
const DODGE: &str = "dodge";
const DROP: &str = "drop";
const ENDURE: &str = "endure";
const EXIT: &str = "exit";
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

pub mod command;
pub use command::*;

pub mod parser;
pub use parser::parse_input;
