use command::CommandReply;

use discord::model::{ChannelId, Event, Message, UserId};
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
    owner: UserId,
}

struct Command(
    String,
    Box<Fn(&Carbot, &Message, &Vec<&str>) -> CommandReply>,
);

impl Carbot {
    pub fn new(owner_id: UserId, token: String, prefix: String) -> Result<Self, discord::Error> {
        let discord = Discord::from_bot_token(token.as_ref())?;

        let (connection, ready) = discord.connect()?;

        let state = State::new(ready);

        let commands = vec![
            Command(
                String::from("ping"),
                Box::new(move |_bot, message, _args| {
                    CommandReply::Message(message.channel_id, String::from("*Pong!*"))
                }),
            ),
            Command(
                String::from("send"),
                Box::new(move |bot, message, args| {
                    if bot.owner != message.author.id {
                        return CommandReply::Message(
                            message.channel_id,
                            String::from("You don't have access to that command!"),
                        );
                    }

                    if args.len() < 2 {
                        return CommandReply::Message(
                            message.channel_id,
                            format!("Not enough arguments.\nUsage: `send <channel_id> <message>`"),
                        );
                    }

                    let channel_id = match args[0].parse::<u64>() {
                        Ok(val) => ChannelId(val),
                        Err(_) => {
                            return CommandReply::Message(
                                message.channel_id,
                                String::from("Please give a valid channel id!"),
                            );
                        }
                    };

                    return CommandReply::Message(channel_id, args[1..].join(" "));
                }),
            ),
            Command(
                String::from("help"),
                Box::new(move |_bot, message, _args| {
                    CommandReply::Message(message.channel_id, String::from("Help yourself!"))
                }),
            ),
        ];

        Ok(Carbot {
            prefix: prefix,
            discord: discord,
            connection: connection,
            state: state,
            messages: VecDeque::new(),
            commands: commands,
            owner: owner_id,
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
                            match cmd.1(&self, &message, &arguments) {
                                CommandReply::Message(channel_id, text) => {
                                    let _ = self.discord.send_message(channel_id, &text, "", false);
                                }
                            }
                            continue 'event_loop;
                        }
                    }

                    eprintln!("[*] Unknown command: {}", command);

                    let _ = self.discord.send_message(
                        message.channel_id,
                        &format!("Unknown command! Try `{}help`.", self.prefix),
                        "",
                        false,
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
