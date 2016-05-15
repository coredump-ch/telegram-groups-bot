//! Implementations of the command handlers.

use telegram_bot::{Message, Chat};
use url::{Url};

use types::Command;
use datastore::{RedisPool, save_group};


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
          /add <name> <invite-url> - Register a new topic group"
          .into())

}


/// Return list of groups.
pub fn handle_groups(command: &Command, _: &Message, _: Option<RedisPool>)
                     -> Option<String> {
    info!("Handled /groups: {}", command);
    Some("Not yet implemented.".into())
}


/// Add a new topic group.
pub fn handle_add(command: &Command, message: &Message, pool: Option<RedisPool>)
                  -> Option<String> {
    info!("Handled /add: {}", command);

    let usage = "Usage: /add <name> <invite-url>\n\
                 Example: /add Bot Development https://telegram.me/...";

    // Rough validation
    if command.params.len() < 2 {
        return Some(format!("Not enough arguments.\n{}", usage));
    }

    // Parse URL
    let url = match command.params.last().map(|e| Url::parse(e).ok()) {
        Some(Some(url)) => url,
        _ => return Some(format!("Bad URL.\n{}", usage)),
    };

    // Get main group name
    let main = match message.chat {
        Chat::Group { id, .. } => id,
        _ => {
            return Some("For technical reasons, this bot can only be used \
                         from within a group.".into());
        }
    };

    // Get topic group name
    let topic = &command.params[..command.params.len()-1].join(" ");

    // Store group
    match save_group(main, &topic, &url, pool.expect("No redis pool passed in.")) {
        Ok(_) => Some("Thanks, group was saved!".into()),
        Err(e) => {
            error!("Could not save group: {:?}", e);
            Some("Error while trying to save group.".into())
        }
    }
}
