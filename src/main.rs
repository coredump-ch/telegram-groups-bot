extern crate telegram_bot;
extern crate conv;
#[macro_use] extern crate log;

mod errors;

use telegram_bot::{Api, ListeningMethod, MessageType, ListeningAction};
use conv::TryFrom;

use errors::CommandParseError;


#[derive(Debug)]
struct Command {
    pub name: String,
    pub params: Vec<String>,
}


/// Parse a text message, return a command if possible
impl<'a> TryFrom<&'a str> for Command {
    type Err = CommandParseError;
    fn try_from(text: &'a str) -> Result<Self, CommandParseError> {

        // Verify if this is actually a command
        if !text.starts_with("/") {
            return Err(CommandParseError::NoCommand);
        }

        // Split text into words iterator
        let mut words = text.split(' ');

        // Parse out name and params
        if let Some(name) = words.next() {
            let params: Vec<String> = words.map(|s| s.into()).collect();
            Ok(Command {
                name: name.into(),
                params: params,
            })
        } else {
            Err(CommandParseError::NoCommand)
        }
    }
}


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
