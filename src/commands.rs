//! Implementations of the command handlers.

use types::Command;


/// A command handler handles commands in a separate thread.
pub struct CommandHandler {
    command: Command,
    handler: Box<Fn(&Command) + Send + Sync>,
}

impl CommandHandler {

    pub fn new(command: Command, handler: Box<Fn(&Command) + Send + Sync>) -> CommandHandler {
        CommandHandler {
            command: command,
            handler: handler,
        }
    }

    pub fn handle(&self) {
        self.handler.call((&self.command,));
    }

}

pub fn handle_debug(command: &Command) {
    info!("Handled command: {}", command);
}

pub fn handle_help(command: &Command) {
    info!("Handled help: {}", command);
}
