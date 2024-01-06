use std::io;

pub mod interpreter;

const PROMPT_ERROR: &str = "Try command again.";
const HERO_PROMPT: &str = "What do you do hero?";

/// A function that ask the user for input and collects it.
///
/// # Returns
/// * `Result<String, String>` - A string that is the user's input, or an error message.
pub fn prompt() -> Result<String, String> {
    println!("{}", HERO_PROMPT);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(i) => i,
        Err(_) => return Err(String::from(PROMPT_ERROR)),
    };
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO pick up here with mocking.

    #[test]
    fn prompt_test() {
        let input = prompt().unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(input, "go north\n");
    }
}
