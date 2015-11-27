//! Implementations of the command handlers.

use types::Command;


/// A command handler handles commands in a separate thread.
pub struct CommandHandler<F> where F: Fn(&Command) {
    command: Command,
    handler: F,
}

impl<F> CommandHandler<F> where F: Fn(&Command) {

    pub fn new(command: Command, handler: F) -> CommandHandler<F> {
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
