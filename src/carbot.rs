use command::{Command, CommandParseError};

use discord::model::{Event, Message};
use discord::{Connection, Discord, State};

use std::collections::VecDeque;
use std::thread;

static MAX_MESSAGES: usize = 1000;

pub struct Carbot {
    prefix: String,
    discord: Discord,
    connection: Connection,
    state: State,
    messages: VecDeque<Message>,
}

impl Carbot {
    pub fn new(token: String, prefix: String) -> Result<Self, discord::Error> {
        let discord = Discord::from_bot_token(token.as_ref())?;

        let (connection, ready) = discord.connect()?;

        let state = State::new(ready);

        Ok(Carbot {
            prefix: prefix,
            discord: discord,
            connection: connection,
            state: state,
            messages: VecDeque::new(),
        })
    }

    pub fn run(mut self) {
        loop {
            let event = match self.connection.recv_event() {
                Ok(event) => event,
                Err(err) => {
                    eprintln!("[*] Received error: {}", err);

                    if let discord::Error::WebSocket(..) = err {
                        let (connection, ready) =
                            self.discord.connect().expect("Reconnection failed");
                        self.connection = connection;
                        self.state = State::new(ready);
                        eprintln!("[*] Reconnection successfull!");
                    } else if let discord::Error::Closed(..) = err {
                        break;
                    }
                    continue;
                }
            };

            self.state.update(&event);

            match event {
                Event::MessageCreate(message) => {
                    self.messages.push_back(message.clone());
                    self.messages.truncate(MAX_MESSAGES);

                    match Command::from_message(&self.prefix, &message) {
                        Ok(command) => command.execute(&self.discord),
                        Err(CommandParseError::NotACommand) => {
                            println!("{}: {}", message.author.name, message.content)
                        }
                        Err(CommandParseError::UnknownCommand) => {
                            eprintln!("Invalid command: {}", message.content);
                            let _ = self.discord.send_message(
                                message.channel_id,
                                &format!("Invalid command, try `{}help`.", self.prefix),
                                "",
                                false,
                            );
                        }
                    }
                }
                Event::MessageDelete {
                    channel_id,
                    message_id,
                } => {
                    for message in self.messages.iter() {
                        if message_id == message.id {
                            let _ = self.discord.send_message(
                                channel_id,
                                &format!("{} deleted: {}", message.author.name, message.content),
                                "",
                                false,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
