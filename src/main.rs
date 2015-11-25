extern crate telegram_bot;
extern crate threadpool;
extern crate conv;
#[macro_use] extern crate log;
extern crate env_logger;

mod types;
mod errors;
mod commands;

use telegram_bot::{Api, ListeningMethod, MessageType, ListeningAction};
use threadpool::ThreadPool;
use conv::TryFrom;

use types::Command;
use commands::CommandHandler;


fn main() {

    // Initialize env logger
    env_logger::init().unwrap();

    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());

    // We want to listen for new updates via LongPoll
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    // Create thread pool for command handlers
    let pool = ThreadPool::new(12);

    // Fetch new updates via long poll method
    listener.listen(|u| {
        if let Some(m) = u.message {

            if let MessageType::Text(text) = m.msg {

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
