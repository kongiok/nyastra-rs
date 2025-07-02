use serenity::model::prelude::{Guild, UnavailableGuild};
use serenity::prelude::Context;

pub async fn handle_guild_create(_ctx: &Context, guild: &Guild, is_new: Option<bool>) {
    if !matches!(is_new, Some(true)) {
        return;
    }

    println!("Bot Process | I'm in {}!", guild.name);
}

pub async fn handle_guild_delete(
    _ctx: &Context,
    incomplete: &UnavailableGuild,
    full: Option<&Guild>,
) {
    let guild_id = incomplete.id;

    let guild_name = match full {
        Some(guild) => guild.name.clone(),
        None => String::from("Unknown (not cached)"),
    };

    if incomplete.unavailable {
        println!(
            "🔌 Guild went offline (可能是 Discord 問題)：[{}] ({})",
            guild_name, guild_id
        );
    } else {
        println!("❌ Bot 被踢出伺服器：[{}] ({})", guild_name, guild_id);
    }
}
