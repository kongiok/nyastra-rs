use dotenvy;
use nyastra::events::handler;
use serenity::{Client, model::gateway::GatewayIntents};
use std::env;
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let app_name = match env::var("APP_NAME") {
        Ok(name) => name.to_string(),
        Err(_) => "Nyastra Bot".to_string(),
    };
    println!("Hello, {}!", app_name);
    let access_token = env::var("DISCORD_BOT_ACCESS_TOKEN").expect("Bot Access | Required token");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::DIRECT_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGE_TYPING
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let mut discord_client = Client::builder(&access_token, intents)
        .event_handler(handler::NyastraHandler)
        .await
        .expect("Bot Access | client does not create successfully");
    if let Err(why) = discord_client.start().await {
        println!("Bot Access | Client error: {why:?}");
    }
}
