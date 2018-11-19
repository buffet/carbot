use discord::model::{Event, Message};
use discord::{Connection, Discord, State};

use std::collections::VecDeque;

static MAX_MESSAGES: usize = 1000;

pub struct Carbot {
    prefix: String,
    discord: Discord,
    connection: Connection,
    state: State,
    messages: VecDeque<Message>,
    commands: Vec<Command>,
}

struct Command(String, Box<Fn(&Discord, &Message, &Vec<&str>)>);

impl Carbot {
    pub fn new(token: String, prefix: String) -> Result<Self, discord::Error> {
        let discord = Discord::from_bot_token(token.as_ref())?;

        let (connection, ready) = discord.connect()?;

        let state = State::new(ready);

        let commands = vec![
            Command(String::from("ping"), Box::new(move |discord, message, _args| {
                let _ = discord.send_message(
                    message.channel_id,
                    "*Pong!*",
                    "",
                    false
                );
            })),
        ];

        Ok(Carbot {
            prefix: prefix,
            discord: discord,
            connection: connection,
            state: state,
            messages: VecDeque::new(),
            commands: commands,
        })
    }

    pub fn run(mut self) {
        'event_loop: loop {
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

                    eprintln!("{}: {}", message.author.name, message.content);

                    if !message.content.starts_with(&self.prefix) || message.author.bot {
                        continue;
                    }

                    let mut split = message.content[self.prefix.len()..].split(" ");
                    let command = split.next().unwrap_or("");
                    let arguments: Vec<&str> = split.collect();

                    for cmd in self.commands.iter() {
                        if &cmd.0 == command {
                            cmd.1(&self.discord, &message, &arguments);
                            continue 'event_loop;
                        }
                    }

                    eprintln!("[*] Unknown command: {}", command);

                    let _ = self.discord.send_message(
                        message.channel_id,
                        &format!("Unknown command! Try `{}help`.", self.prefix),
                        "",
                        false
                    );
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
