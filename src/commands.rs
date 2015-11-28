//! Implementations of the command handlers.

use types::Command;


pub type BoxedHandler = Box<Fn(&Command) -> Option<String> + Send + Sync>;


/// A command handler handles commands in a separate thread.
pub struct CommandHandler {
    command: Command,
    handler: BoxedHandler,
}

impl CommandHandler {

    pub fn new(command: &Command,
               handler: BoxedHandler)
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
