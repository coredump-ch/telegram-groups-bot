//! Implementations of the command handlers.

use types::Command;


/// Log the command, don't do anything else.
pub fn handle_log(command: &Command) -> Option<String> {
    info!("Handled command: {}", &command);
    None
}


/// Return help output.
pub fn handle_help(command: &Command) -> Option<String> {
    info!("Handled /help: {}", command);
    Some("Available commands:\n\n \
          /help - show this help\n \
          /groups - list all available groups, along with the invite link"
          .into())

}


/// Return list of groups.
pub fn handle_groups(command: &Command) -> Option<String> {
    info!("Handled /groups: {}", command);
    Some("Not yet implemented.".into())
}
