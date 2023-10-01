use std::{
    env,
    sync::{Arc, RwLock},
};

use serenity::{
    async_trait,
    prelude::{EventHandler, GatewayIntents},
    Client,
};

use methoxymethyl_isothiocyanate::utils::load_env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    load_env();
    // Configure the client with your Discord bot token in the environment.
    let token = env::args().nth(1).expect("Expected a bot token");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    // Build our client.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
