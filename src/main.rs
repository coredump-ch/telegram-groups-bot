//! # Telegram Groups Bot
//!
//! This bot can be used to manage multiple Telegram topic group chats.
//! Admins can register topic channels, which the users can then list.
//! The users can also request the invite link for one of the topic channels.
//!
//! ## Command API
//!
//! - `/help` Show help
//! - `/groups` Show list of available topic groups, along with the invite link
//! - `/add <name> <invite-link>` Register a new topic group
//!
//! ## Implementation Details
//!
//! This bot builds on top of the [telegram-bot][0] crate. It listens to the Telegram API through
//! LongPoll.
//!
//! When a message comes in, it is first parsed into a `Command`. If that worked out, the command
//! is processed by a command handler in a thread pool.
//!
//! [0]: https://crates.io/crates/telegram-bot

extern crate telegram_bot;
extern crate threadpool;
extern crate conv;
#[macro_use] extern crate log;
extern crate env_logger;

pub mod types;
pub mod errors;
pub mod commands;

use std::process::exit;

use telegram_bot::{Api, Listener, ListeningMethod, Message, MessageType, ParseMode, ListeningAction};
use threadpool::ThreadPool;
use conv::TryFrom;

use types::Command;


/// Initialize and return a `telegram_bot::Listener` instance.
fn get_listener(api: &Api) -> Listener {
    match api.get_me() {
        Ok(user) => println!("Starting {}...", user.first_name),
        Err(e) => {
            println!("Error: Could not fetch information about Telegram bot.");
            println!("  Error Details: {:?}", e);
            println!("  Maybe check your TELEGRAM_BOT_TOKEN env var?");
            exit(1);
        },
    }

    api.listener(ListeningMethod::LongPoll(None))

}


fn main() {

    // Initialize env logger
    env_logger::init().unwrap();

    // Get Telegram Api listener
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap_or_else(|_| {
        println!("Error: TELEGRAM_BOT_TOKEN env var missing");
        exit(1);
    });
    let mut listener = get_listener(&api);

    // Get own username
    let username = match api.get_me() {
        Ok(user) => user.username,
        Err(e) => {
            println!("Error: Could not fetch information about Telegram bot.");
            println!("  Error Details: {:?}", e);
            println!("  Maybe check your TELEGRAM_BOT_TOKEN env var?");
            exit(1);
        },
    };

    // Create thread pool for command handlers
    let pool = ThreadPool::new(12);

    // Fetch new updates via long poll method
    listener.listen(|u| {

        // Dispatch messages
        if let Some(m) = u.message {

            // Get chat id
            let chat_id = m.chat.id();

            // Get copy of API and message
            let api_clone = api.clone();
            let msg_clone = m.clone();

            // Process text messages
            if let MessageType::Text(text) = m.msg {

                // Dispatch command handlers
                let command = Command::try_from(&*text);
                match command {
                    Ok(ref cmd) if cmd.receiver.is_some() && cmd.receiver != username => {
                        debug!("Ignored command, not directed at me: {:?}", cmd);
                    },
                    Ok(cmd) => {
                        debug!("Command: {:?}", cmd);

                        // Choose handler
                        let handler: Box<Fn(&Command, &Message) -> Option<String> + Send> = match &*cmd.name {
                            "help" => Box::new(commands::handle_help),
                            "groups" => Box::new(commands::handle_groups),
                            "add" => Box::new(commands::handle_add),
                            _ => Box::new(commands::handle_log),
                        };

                        // Run the handler in a separate thread
                        pool.execute(move || {
                            if let Some(reply) = handler(&cmd, &msg_clone) {
                                debug!("Return msg: {}", reply);
                                let parse_mode = Some(ParseMode::Markdown);
                                let disable_web_page_preview = Some(true);
                                api_clone.send_message(chat_id, reply,
                                                       parse_mode, disable_web_page_preview,
                                                       None, None);
                            };
                        });
                    }
                    Err(_) => debug!("No command."),
                }

            } else {
                println!("Other msg was {:?}", m.msg);
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });

}
