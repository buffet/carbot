extern crate env_logger;
extern crate kankyo;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serenity;

mod commands;

use serenity::{
    framework::StandardFramework,
    http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};

use std::{collections::HashSet, env};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _resume: ResumedEvent) {
        info!("Resumed");
    }
}

fn main() {
    kankyo::load().expect("Failed to load .env file");
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let prefix = env::var("DISCORD_PREFIX").unwrap_or(String::from("./"));

    let mut client = Client::new(&token, Handler).expect("Error creating client");
    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(err) => panic!("Couldn't get app info: {:?}", err),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.owners(owners)
                    .on_mention(true)
                    .prefix(&prefix)
                    .case_insensitivity(true)
            }).command("ping", |c| c.cmd(commands::misc::ping)),
    );

    if let Err(err) = client.start() {
        error!("Client error: {:?}", err);
    }
}
