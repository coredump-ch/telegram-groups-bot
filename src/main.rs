extern crate telegram_bot;
extern crate threadpool;
extern crate conv;
#[macro_use] extern crate log;
extern crate env_logger;

mod types;
mod errors;
mod commands;

use std::process::exit;

use telegram_bot::{Api, Listener, ListeningMethod, MessageType, ListeningAction};
use threadpool::ThreadPool;
use conv::TryFrom;

use types::Command;
use commands::CommandHandler;


/// Initialize and return a `telegram_bot::Listener` instance.
fn get_listener() -> Listener {
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
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
    let mut listener = get_listener();

    // Create thread pool for command handlers
    let pool = ThreadPool::new(12);

    // Fetch new updates via long poll method
    listener.listen(|u| {

        // Dispatch messages
        if let Some(m) = u.message {

            // Process text messages
            if let MessageType::Text(text) = m.msg {

                // Dispatch command handlers
                let command = Command::try_from(&*text);
                match command {
                    Ok(cmd) => {
                        debug!("Command: {:?}", cmd);
                        let handler = commands::LogHandler { command: cmd.clone() };
                        pool.execute(move || {
                            handler.handle();
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
