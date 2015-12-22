//! Implementations of the command handlers.

use types::Command;


pub fn handle_log(command: &Command) -> Option<String> {
    info!("Handled command: {}", &command);
    None
}


pub fn handle_help(command: &Command) -> Option<String> {
    info!("Handled help: {}", command);
    Some("Available commands:\n\n \
          /help - show this help\n \
          /groups - list all available groups, along with the invite link"
          .into())

}
