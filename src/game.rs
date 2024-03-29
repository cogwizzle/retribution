use crate::{migration, plugin::PLUGIN_OUTPUT};
use std::io;

pub mod interpreter;
pub mod map;
pub mod state;

/// Prompt error message.
const PROMPT_ERROR: &str = "Try command again.";
/// Prompt message.
const HERO_PROMPT: &str = "What do you do hero?";

/// A trait that defines a function to read a line.
pub trait LineReader {
    /// A function that reads a line from the user.
    ///
    /// # Arguments
    /// * `buf` - A mutable reference to a string.
    /// * `self` - A mutable reference to a `io::Stdin`.
    ///
    /// # Returns
    /// * `Result<usize, io::Error>` - A usize that is the length of the string, or an error.
    fn read_line(&mut self, buf: &mut String) -> Result<usize, io::Error>;
}

impl LineReader for io::Stdin {
    fn read_line(&mut self, buf: &mut String) -> Result<usize, io::Error> {
        io::Stdin::read_line(self, buf)
    }
}

/// A function that ask the user for input and collects it.
///
/// # Arguments
/// * `reader` - A mutable reference to a `io::Stdin`.
///
/// # Returns
/// * `Result<String, &'a str>` - A string that is the user's input, or an error message.
pub fn prompt<'a>(reader: &'a mut dyn LineReader) -> Result<String, &'a str> {
    println!("{}", HERO_PROMPT);
    let mut input = String::new();
    reader.read_line(&mut input).map_err(|_| PROMPT_ERROR)?;
    Ok(input)
}

/// Function to run before the game initializes.
///
/// # Returns
/// * `Result<(), &'static str>` - A result that is either Ok or Err.
pub fn init() -> Result<(), &'static str> {
    // Set up the database.
    migration::map::migrate_up(None)
}

/// Function to run after the game ends.
///
/// # Returns
/// * `Result<(), &'static str>` - A result that is either Ok or Err.
pub fn tear_down() -> Result<(), &'static str> {
    std::fs::remove_file(PLUGIN_OUTPUT).map_err(|_| "Failed to remove plugin file.")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A mock struct that implements the LineReader trait.
    struct MockReader {
        input: String,
    }

    /// Implement the LineReader trait for the MockReader struct.
    impl LineReader for MockReader {
        fn read_line(&mut self, buf: &mut String) -> Result<usize, io::Error> {
            buf.push_str(&self.input);
            Ok(self.input.len())
        }
    }

    /// Test the prompt function.
    #[test]
    fn prompt_test() {
        let mut reader = MockReader {
            input: String::from("go north\n"),
        };
        let input = prompt(&mut reader).unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(input, "go north\n");
    }

    struct ErrMockReader {}

    impl LineReader for ErrMockReader {
        fn read_line(&mut self, _buf: &mut String) -> Result<usize, io::Error> {
            Err(io::Error::new(io::ErrorKind::Other, "Mock Error"))
        }
    }

    /// Test the prompt function with an error.
    #[test]
    fn prompt_error_test() {
        let mut reader = ErrMockReader {};
        let input = prompt(&mut reader);
        assert_eq!(input, Err(PROMPT_ERROR));
    }
}
