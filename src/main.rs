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
//! - `/add <name> <invite-url>` Register a new topic group
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
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate url;
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate quick_error;

pub mod types;
pub mod errors;
pub mod commands;
pub mod datastore;

use std::process::exit;
use std::time::Duration;

use telegram_bot::{Api, Listener, ListeningMethod, Message, MessageType, ParseMode, ListeningAction};
use threadpool::ThreadPool;
use conv::TryFrom;
use r2d2_redis::RedisConnectionManager;

use types::Command;
use datastore::RedisPool;


/// Initialize and return a `telegram_bot::Listener` instance.
fn get_listener(api: &Api) -> Listener {
    match api.get_me() {
        Ok(user) => println!("Starting {}...", user.first_name),
        Err(e) => {
            println!("Error: Could not fetch information about Telegram bot.");
            println!("  Error details: {:?}", e);
            println!("  Maybe check your TELEGRAM_BOT_TOKEN env var?");
            exit(1);
        },
    }

    api.listener(ListeningMethod::LongPoll(None))
}


/// Get a Redis database connection pool
fn get_redis_pool() -> RedisPool {
    let config = r2d2::Config::builder()
        .pool_size(4)
        .initialization_fail_fast(true)
        .connection_timeout(Duration::from_secs(5))
        .build();
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    match r2d2::Pool::new(config, manager) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Error: Could not initialize Redis connection pool.");
            println!("  Error details: {:?}", e);
            println!("  Is the Redis server running?");
            exit(1);
        },
    }
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

    // Get connection pool
    let redispool = get_redis_pool();

    // Get own username
    let username = match api.get_me() {
        Ok(user) => user.username,
        Err(e) => {
            println!("Error: Could not fetch information about Telegram bot.");
            println!("  Error details: {:?}", e);
            println!("  Maybe check your TELEGRAM_BOT_TOKEN env var?");
            exit(1);
        },
    };

    // Create thread pool for command handlers
    let threadpool = ThreadPool::new(12);

    println!("Up and running!");

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
                        type Handler = Box<Fn(&Command, &Message, Option<RedisPool>)
                                              -> Option<String> + Send>;
                        let handler: Handler = match &*cmd.name {
                            "help" => Box::new(commands::handle_help),
                            "groups" => Box::new(commands::handle_groups),
                            "add" => Box::new(commands::handle_add),
                            _ => Box::new(commands::handle_log),
                        };

                        // Run the handler in a separate thread
                        let redispool_clone = redispool.clone();
                        threadpool.execute(move || {
                            if let Some(reply) = handler(&cmd, &msg_clone,
                                                         Some(redispool_clone)) {
                                debug!("Return msg: {}", reply);
                                let parse_mode = Some(ParseMode::Markdown);
                                let disable_web_page_preview = Some(true);
                                let result = api_clone.send_message(chat_id, reply,
                                                                    parse_mode,
                                                                    disable_web_page_preview,
                                                                    None, None);
                                if let Err(e) = result {
                                    error!("Could not send message: {:?}", e);
                                }
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
