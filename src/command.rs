use discord::model::{ChannelId, Message, MessageId, User};
use discord::Discord;

pub struct Command {
    channel_id: ChannelId,
    message_id: MessageId,
    user: User,
    command_type: CommandType,
}

enum CommandType {
    Ping,
}

pub enum CommandParseError {
    NotACommand,
    UnknownCommand,
}

impl Command {
    pub fn from_message(prefix: &str, message: &Message) -> Result<Self, CommandParseError> {
        if !message.content.starts_with(prefix) || message.author.bot {
            return Err(CommandParseError::NotACommand);
        }

        if message.author.bot {
            return Err(CommandParseError::NotACommand);
        }

        let mut split = message.content.split(" ");
        let command = split.next().unwrap_or("");
        let arguments: Vec<&str> = split.collect();

        let command_type = match &command[prefix.len()..] {
            "ping" => CommandType::Ping,
            _ => return Err(CommandParseError::UnknownCommand),
        };

        Ok(Command {
            channel_id: message.channel_id,
            message_id: message.id,
            user: message.author.clone(),
            command_type: command_type,
        })
    }

    pub fn execute(self, discord: &Discord) {
        match self.command_type {
            CommandType::Ping => {
                let _ = discord.send_message(self.channel_id, "*Pong!*", "", false);
            }
        }
    }
}
