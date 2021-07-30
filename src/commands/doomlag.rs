use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        Args, CommandResult, 
        macros::command,
    },
};

use crate::utils::*;
use crate::fabi_error::FabiError;

#[command]
#[only_in(guilds)]
#[description = "Envoi de TOUT les Fabiens (sauf celui qui a fait la commande) au Goulag !"]
pub async fn doomlag(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;
    let mut sender = guild_id.member(&ctx.http, msg.author.id).await?;

    // Check if the sender has the required permissions
    if let Ok(perms) = sender.permissions(&ctx).await {
        if !perms.manage_messages() {
            msg.channel_id.say(&ctx.http, ":slight_smile:").await?;
            return Ok(());
        }
    }

    let gulag_role = get_role(ctx, guild_id, "Fabien du goulag").await?;
    let mut members = guild_id.members(&ctx.http, None, None).await?;

    // Countdown before doomlag
    msg.channel_id.say(&ctx.http, format!("everyone, le Doomlag a été initié par {}", msg.author)).await?;
    for i in (1..=3).rev() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        msg.channel_id.say(&ctx.http, i.to_string()).await?;
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
    msg.channel_id.say(&ctx.http, "Que vous pourissiez tous au Goulag").await?;
    
    for member in members.iter_mut() {
        member.add_role(&ctx.http, &gulag_role).await?;
    }

    sender.remove_role(&ctx.http, gulag_role).await?;

    Ok(())
}
