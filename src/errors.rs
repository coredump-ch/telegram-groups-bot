//! Errors used in this bot implementation.

use std::error;
use std::fmt;


/// An enumeration of possible erorrs that can happen
/// during the parsing of a message.
#[derive(Debug)]
pub enum CommandParseError {
    NoCommand,  // The text is not actually a command
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandParseError::NoCommand => write!(f, "NoCommand")
        }
    }
}

impl error::Error for CommandParseError {
    fn description(&self) -> &str {
        &"Could not parse the command."
    }
}
