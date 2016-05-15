//! Types used in this bot implementation.

use std::fmt;

use errors::CommandParseError;


/// A stateless command.
///
/// A command consists of a name and a list of parameters.
///
/// # Example
///
/// The string
///
///     /join channel1 channel2
///
/// ...will become...
///
///     Command {
///         name: "join",
///         params: ["channel1", "channel2"]
///     }
#[derive(Debug, Clone)]
pub struct Command {
    /// Command name, without a leading slash character.
    pub name: String,
    /// List of command parameters. May be empty.
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
        if let Some(command) = words.next() {

            // Strip leading slash
            let name: String = command[1..].into();

            // If name is an empty string, it's not valid
            if name.len() == 0 {
                return Err(CommandParseError::NoCommand);
            }

            // Parse parameters
            let params: Vec<String> = words.map(|s| s.into()).collect();

            // Return command
            Ok(Command {
                name: name.into(),
                params: params,
            })

        } else {
            Err(CommandParseError::NoCommand)
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{} {}", self.name, self.params.join(" "))
    }
}


#[cfg(test)]
mod tests {
    use super::Command;
    use conv::TryFrom;

    #[test]
    fn command_parse_str_ok() {
        let text_simple: String = "/help".into();
        let text_params = "/list all";

        let command_simple = Command::try_from(&*text_simple).unwrap();
        let command_params = Command::try_from(text_params).unwrap();

        assert_eq!(command_simple.name, "help");
        assert_eq!(command_simple.params, Vec::<String>::new());
        assert_eq!(command_params.name, "list");
        assert_eq!(command_params.params, vec!["all"]);
    }

    #[test]
    fn command_parse_string_err() {
        let texts = vec![
            "",
            "/",
            "no initial slash",
            " /preceding space",
            "/ params but no name",
        ];

        for &text in texts.iter() {
            assert!(Command::try_from(text).is_err());
        }
    }

}
