extern crate discord;

mod carbot;
mod command;

use carbot::Carbot;

use std::env;
use std::process;

fn main() {
    let prefix = env::var("DISCORD_PREFIX").unwrap_or(String::from("./"));
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let bot = match Carbot::new(token, prefix) {
        Ok(bot) => bot,
        Err(err) => {
            eprintln!("Failed to initialize bot: {:?}", err);
            process::exit(1);
        }
    };

    bot.run();
}
