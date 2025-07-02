pub use crate::events::messages;
use serenity::model::{channel::Message, gateway::Ready, guild::Member};
use serenity::{async_trait, client::Context, client::EventHandler};
pub struct NyastraHandler;

#[async_trait]
impl EventHandler for NyastraHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        messages::handle_message_event(&ctx, &msg).await;
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot Process | {} is connected!", ready.user.name);
    }
}
