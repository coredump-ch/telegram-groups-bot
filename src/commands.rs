//! Implementations of the command handlers.

use types::Command;


/// A CommandHandler that can be run in a background thread.
pub trait CommandHandler: Send + Sync {
    fn handle(&self);
}


/// A simple handler that just logs the command.
pub struct LogHandler {
    pub command: Command,
}

impl CommandHandler for LogHandler {
    fn handle(&self) {
        info!("Handled command: {:?}", &self.command);
    }
}
