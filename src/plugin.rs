//! # Plugin
//!
//! Handles the plugin interface for the game.
use std::thread;
use serde::{Deserialize, Serialize};
use crate::game::state;

/// The version of the plugin.
const VERSION: &str = "0.1.0";
/// The path to the plugin file.
const PLUGIN_OUTPUT: &str = "~/ret-plugin.json";

/// A struct that represents the output of the plugin.
#[derive(Serialize, Deserialize)]
struct PluginOutput {
    /// The version of the plugin.
    pub version: String,
    /// The game state to write to the plugin file.
    pub game_state: state::GameState,
}

impl PluginOutput {
    /// A function that creates a new PluginOutput.
    ///
    /// # Arguments
    /// * `game_state` - The game state to write to the plugin file.
    ///
    /// # Returns
    /// * `PluginOutput` - A new PluginOutput.
    pub fn new(game_state: state::GameState) -> PluginOutput {
        PluginOutput {
            version: VERSION.to_string(),
            game_state,
        }
    }
}

/// A struct that writes the state to the plugin file.
pub struct StateWriter {
    /// The path to the plugin file.
    pub output_file: String,
}

impl StateWriter {
    /// A function that creates a new StateWriter.
    ///
    /// # Arguments
    /// * `path` - The path to the plugin file.
    ///
    /// # Returns
    /// * `StateWriter` - A new StateWriter.
    ///
    /// # Examples
    /// ```
    /// use retribution::plugin;
    /// use std::borrow::Cow;
    ///
    /// let path = String::from("test.json");
    /// let state_writer = plugin::StateWriter::new(Some(path));
    /// ```
    pub fn new(path: Option<String>) -> StateWriter {
        let path = match path {
            Some(p) => p,
            None => PLUGIN_OUTPUT.to_string(),
        };
        let path = path.replace("~", std::env::var("HOME").unwrap().as_str());
        StateWriter { output_file: path }
    }

    /// Writes the state to the plugin file.
    ///
    /// # Arguments
    /// * `state` - The state to write to the plugin file.
    ///
    /// # Returns
    /// * `Result<(), String>` - The result of writing the state to the plugin file.
    pub fn write_state(&self, state: state::GameState) -> Result<(), String> {
        println!("write_state");
        // spawn a thread to write the state to the plugin file.
        let output_file = self.output_file.clone();
        let state_clone = state.clone();
        thread::spawn(move || {
            let plugin_output = PluginOutput::new(state_clone);
            let json = serde_json::to_string(&plugin_output).unwrap();
            std::fs::write(output_file, json).unwrap();
        }).join().map_err(|_| "Failed to write state to plugin file.".to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_writer_write_state_test() {
        let game_state = state::GameState::new();
        let state_writer = StateWriter::new(Some("test.json".to_string()));
        let results = state_writer.write_state(game_state);
        std::fs::remove_file("test.json").unwrap();
        assert!(results.is_ok());
    }

    /// Test the plugin output constructor.
    #[test]
    fn plugin_output_test() {
        let game_state = state::GameState::new();
        let plugin_output = PluginOutput::new(game_state.clone());
        assert_eq!(plugin_output.version, VERSION);
    }
}
