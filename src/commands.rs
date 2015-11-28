//! Implementations of the command handlers.

use types::Command;


/// A command handler handles commands in a separate thread.
pub struct CommandHandler {
    command: Command,
    handler: Box<Fn(&Command) -> Option<String> + Send + Sync>,
}

impl CommandHandler {

    pub fn new(command: &Command,
               handler: Box<Fn(&Command) -> Option<String> + Send + Sync>)
               -> CommandHandler {
        CommandHandler {
            command: command.clone(),
            handler: handler,
        }
    }

    pub fn handle(&self) -> Option<String> {
        self.handler.call((&self.command,))
    }

}

pub fn handle_debug(command: &Command) -> Option<String> {
    info!("Handled command: {}", command);
    None
}

pub fn handle_help(command: &Command) -> Option<String> {
    info!("Handled help: {}", command);
    Some("Available commands:\n\n \
         /help - show this help\n \
         /groups - list all available groups, along with the invite link"
         .into())
}

pub fn handle_groups(command: &Command) -> Option<String> {
    info!("Handled /groups: {}", command);
    Some("Not yet implemented.".into())
}
