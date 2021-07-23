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
pub async fn doomlag(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;

    let gulag_role = get_role(ctx, guild_id, "Fabien du goulag").await?;

    // Countdown before doomlag
    msg.channel_id.say(&ctx.http, format!("everyone, le Doomlag a été initié par {}", msg.author)).await?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    msg.channel_id.say(&ctx.http, "3").await?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    msg.channel_id.say(&ctx.http, "2").await?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    msg.channel_id.say(&ctx.http, "1").await?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    msg.channel_id.say(&ctx.http, "Que vous pourissiez tous au Goulag").await?;
    
    let mut members = guild_id.members(&ctx.http, None, None).await?;
    println!("{:?}", members);
    for member in members.iter_mut() {
        member.add_role(&ctx.http, &gulag_role).await?;
    }

    let mut sender = guild_id.member(&ctx.http, msg.author.id).await?;
    sender.remove_role(&ctx.http, gulag_role).await?;

    Ok(())
}
