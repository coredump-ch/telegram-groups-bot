extern crate telegram_bot;
#[macro_use] extern crate log;

use telegram_bot::{Api, ListeningMethod, MessageType, ListeningAction};


#[derive(Debug)]
struct Command {
    pub name: String,
    pub params: Vec<String>,
}


/// Parse a text message, return a command if possible
/// TODO: Do this via a From<&str> implementation
fn parse_msg(text: String) -> Option<Command> {
    if !text.starts_with("/") {
        return None;
    }

    let mut words = text.split(' ');
    if let Some(name) = words.next() {
        let params: Vec<String> = words.map(|s| s.into()).collect();
        Some(Command {
            name: name.into(),
            params: params,
        })
    } else {
        None
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

                let command = parse_msg(text);
                match command {
                    Some(cmd) => println!("Command: {:?}", cmd),
                    None => println!("No command."),
                }

            } else {
                println!("Other msg was {:?}", m.msg);
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });

}
