use crate::utils::commands::parse_command;
use serenity::client::Context;
use serenity::model::channel::Message;

pub async fn handle_message_event(ctx: &Context, msg: &Message) {
    let command_prefix = std::env::var("COMMAND_PREFIX").unwrap_or("!".to_string());
    let (cmd, args) = match parse_command(&msg.content, &command_prefix) {
        Some(pair) => pair,
        None => return,
    };
    println!("Command: {} {:?}", cmd, args);
    match cmd.to_lowercase().as_str() {
        "ping" => {
            msg.channel_id.say(&ctx.http, "Pong!").await.ok();
            return;
        }
        _ => {
            msg.channel_id.say(&ctx.http, "Unknown Command").await.ok();
            return;
        }
    }
}
