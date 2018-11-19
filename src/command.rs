use discord::model::Message;

pub enum Command {
    Ping,
}

pub enum CommandParseError {
    NotACommand,
    UnknownCommand,
    InvalidArgument,
}

impl Command {
    pub fn from_message(prefix: &str, message: &Message) -> Result<Self, CommandParseError> {
        if !message.content.starts_with(prefix) {
            return Err(CommandParseError::NotACommand);
        }

        let mut split = message.content.split(" ");
        let command = split.next().unwrap_or("");
        let argument = split.next().unwrap_or("");

        match &command[prefix.len()..] {
            "ping" => Ok(Command::Ping),
            _ => Err(CommandParseError::UnknownCommand),
        }
    }

    pub fn execute(self) {
        eprintln!("Not yet implemented");
    }
}
