//! Implementations of the command handlers.

use types::Command;


pub fn handle_log(command: &Command) -> Option<String> {
    info!("Handled command: {}", &command);
    None
}
