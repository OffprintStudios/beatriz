//! # Beatriz
//! ## A chat bot for the official Offprint discord server.
//! 
//! TODO: Actually write the documentation for this.

extern crate serenity;
extern crate tokio;
extern crate dotenv;
#[macro_use] extern crate dotenv_codegen;

mod admin;
mod general;
mod fun;

use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;

use general::GENERAL_GROUP;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = dotenv!("DISCORD_TOKEN");
    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}