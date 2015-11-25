extern crate telegram_bot;
extern crate conv;
#[macro_use] extern crate log;

mod types;
mod errors;

use telegram_bot::{Api, ListeningMethod, MessageType, ListeningAction};
use conv::TryFrom;

use types::Command;


fn main() {

    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());

    // We want to listen for new updates via LongPoll
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    // Fetch new updates via long poll method
    listener.listen(|u| {
        if let Some(m) = u.message {

            if let MessageType::Text(text) = m.msg {

                let command = Command::try_from(&*text);
                match command {
                    Ok(cmd) => println!("Command: {:?}", cmd),
                    Err(_) => println!("No command."),
                }

            } else {
                println!("Other msg was {:?}", m.msg);
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });

}
