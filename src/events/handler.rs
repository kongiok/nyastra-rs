pub use crate::events::messages;
use serenity::model::{channel::Message, gateway::Ready, guild::Guild};
use serenity::{async_trait, client::Context, client::EventHandler};
pub struct NyastraHandler;
use crate::events::guilds;
use serenity::all::UnavailableGuild;

#[async_trait]
impl EventHandler for NyastraHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        messages::handle_message_event(&ctx, &msg).await;
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot Process | {} is connected!", ready.user.name);
    }
    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: Option<bool>) {
        guilds::handle_guild_create(&ctx, &guild, is_new).await;
    }

    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {
        guilds::handle_guild_delete(&ctx, &incomplete, full.as_ref()).await;
    }
}
