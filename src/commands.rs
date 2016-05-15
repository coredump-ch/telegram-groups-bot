//! Implementations of the command handlers.

use telegram_bot::Message;

use types::Command;
use datastore::RedisPool;


/// Log the command, don't do anything else.
pub fn handle_log(command: &Command, _: &Message, _: Option<RedisPool>)
                  -> Option<String> {
    info!("Handled command: {}", &command);
    None
}


/// Return help output.
pub fn handle_help(command: &Command, _: &Message, _: Option<RedisPool>)
                   -> Option<String> {
    info!("Handled /help: {}", command);
    Some("Available commands:\n\n \
          /help - show this help\n \
          /groups - list all available groups, along with the invite link\n \
          /add <name> <invite-link> - Register a new topic group"
          .into())

}


/// Return list of groups.
pub fn handle_groups(command: &Command, _: &Message, _: Option<RedisPool>)
                     -> Option<String> {
    info!("Handled /groups: {}", command);
    Some("Not yet implemented.".into())
}


/// Add a new topic group.
pub fn handle_add(command: &Command, _: &Message, _: Option<RedisPool>)
                  -> Option<String> {
    info!("Handled /add: {}", command);
    Some("Not yet implemented.".into())
}
