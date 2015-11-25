//! Types used in this bot implementation.

use errors::CommandParseError;


#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub params: Vec<String>,
}


/// Parse a text message, return a command if possible
impl<'a> ::conv::TryFrom<&'a str> for Command {
    type Err = CommandParseError;
    fn try_from(text: &'a str) -> Result<Self, CommandParseError> {

        // Verify if this is actually a command
        if !text.starts_with("/") {
            return Err(CommandParseError::NoCommand);
        }

        // Split text into words iterator
        let mut words = text.split(' ');

        // Parse out name and params
        if let Some(name) = words.next() {
            let params: Vec<String> = words.map(|s| s.into()).collect();
            Ok(Command {
                name: name.into(),
                params: params,
            })
        } else {
            Err(CommandParseError::NoCommand)
        }
    }
}
