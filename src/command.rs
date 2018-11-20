use discord::model::ChannelId;

pub enum CommandReply {
    Message(ChannelId, String),
}
