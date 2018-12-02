command!(ping(_context, message) {
    info!("Got `ping`");

    let mut reply = message.channel_id.say("Pong!")?;
    let ts = reply.timestamp;
    let _ = reply.edit(|m| m.content(
        format!("Pong! {}ms", (ts - message.timestamp).num_milliseconds())
    ));
});

