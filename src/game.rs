use std::io;
use crate::migration;

pub mod interpreter;
pub mod state;
pub mod map;

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
/// * `Result<String, String>` - A string that is the user's input, or an error message.
pub fn prompt(reader: &mut dyn LineReader) -> Result<String, String> {
    println!("{}", HERO_PROMPT);
    let mut input = String::new();
    match reader.read_line(&mut input) {
        Ok(i) => i,
        Err(_) => return Err(String::from(PROMPT_ERROR)),
    };
    Ok(input)
}

pub fn init() -> Result<(), &'static str> {
    migration::map::migrate_up(None)
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
}
