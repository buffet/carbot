extern crate discord;

mod carbot;
mod command;

use carbot::Carbot;

use discord::model::UserId;

use std::env;
use std::process;

fn main() {
    let prefix = env::var("DISCORD_PREFIX").unwrap_or(String::from("./"));
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");
    let owner_id = UserId(
        env::var("DISCORD_OWNER")
            .unwrap_or(String::from("0"))
            .parse::<u64>()
            .expect("Invalid DISCORD_OWNER"),
    );

    let bot = match Carbot::new(owner_id, token, prefix) {
        Ok(bot) => bot,
        Err(err) => {
            eprintln!("Failed to initialize bot: {:?}", err);
            process::exit(1);
        }
    };

    bot.run();
}
