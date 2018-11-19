extern crate discord;

use discord::model::{ChannelId, Event, Message, MessageId, User};
use discord::{Discord, State};

use std::env;

struct Request {
    user: User,
    command: Command,
    channel_id: ChannelId,
    message_id: MessageId,
}

enum Command {
    Ping,
}

enum CommandParseError {
    NotACommand,
    UnknownCommand,
    InvalidArgument,
}

impl Request {
    fn from_message(message: &Message) -> Result<Self, CommandParseError> {
        Command::parse(message).map(|command| Request {
            command: command,
            user: message.author.clone(),
            channel_id: message.channel_id,
            message_id: message.id,
        })
    }
}

impl Command {
    fn parse(message: &Message) -> Result<Self, CommandParseError> {
        if !message.content.starts_with(PREFIX) {
            return Err(CommandParseError::NotACommand);
        }

        println!("{}", message.content);

        return Err(CommandParseError::NotACommand);
    }
}

static PREFIX: &'static str = "./";

fn main() {
    let discord =
        Discord::from_bot_token(&env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found"))
            .expect("Failed to login");

    let (mut connection, ready) = discord.connect().expect("Connection failed");
    eprintln!("[*] Bot ready!");

    let mut state = State::new(ready);

    loop {
        let event = match connection.recv_event() {
            Ok(event) => event,
            Err(err) => {
                eprintln!("[*] Received error: {:?}", err);

                if let discord::Error::WebSocket(..) = err {
                    let (new_connection, ready) = discord.connect().expect("Connection failed");
                    connection = new_connection;
                    state = State::new(ready);
                    eprintln!("[*] Reconnected successfully!")
                } else if let discord::Error::Closed(..) = err {
                    break;
                }
                continue;
            }
        };

        state.update(&event);

        match event {
            Event::MessageCreate(message) => match Request::from_message(&message) {
                _ => {}
            },
            _ => {}
        }
    }
}
