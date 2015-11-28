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
//!
//! ## Implementation Details
//!
//! This bot builds on top of the [telegram-bot][0] crate. It listens to the Telegram API through
//! LongPoll.
//!
//! When a message comes in, it is first parsed into a `Command`. If that worked out, the command
//! is dispatched to a `CommandHandler`.
//!
//! All `CommandHandler`s run in a thread pool, so that they don't block the entire bot.
//!
//! [0]: https://crates.io/crates/telegram-bot

#![feature(core)]
#![feature(unboxed_closures)]

extern crate telegram_bot;
extern crate threadpool;
extern crate conv;
#[macro_use] extern crate log;
extern crate env_logger;

pub mod types;
pub mod errors;
pub mod commands;

use std::process::exit;

use telegram_bot::{Api, Listener, ListeningMethod, MessageType, ListeningAction};
use threadpool::ThreadPool;
use conv::TryFrom;

use types::Command;
use commands::CommandHandler;


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
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    let mut listener = get_listener(&api);

    // Create thread pool for command handlers
    let pool = ThreadPool::new(12);

    // Fetch new updates via long poll method
    listener.listen(|u| {

        // Dispatch messages
        if let Some(m) = u.message {

            // Get chat id
            let chat_id = m.chat.id();

            // Process text messages
            if let MessageType::Text(text) = m.msg {

                // Dispatch command handlers
                let command = Command::try_from(&*text);
                match command {
                    Ok(cmd) => {
                        debug!("Command: {:?}", cmd);

                        // Build a CommandHandler
                        let handler = match &*cmd.name {
                            "help" => commands::CommandHandler::new(cmd.clone(), Box::new(commands::handle_help)),
                            "groups" => commands::CommandHandler::new(cmd.clone(), Box::new(commands::handle_groups)),
                            _ => commands::CommandHandler::new(cmd.clone(), Box::new(commands::handle_debug)),
                        };

                        // Run the handler in a separate thread
                        let api_clone = api.clone();
                        pool.execute(move || {
                            if let Some(msg) = handler.handle() {
                                debug!("Msg was: {}", msg);
                                api_clone.send_message(chat_id, msg, None, None, None);
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
