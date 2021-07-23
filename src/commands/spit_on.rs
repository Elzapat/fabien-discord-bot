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
pub async fn spit_on(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the ID of the guild
    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;

    let sender = guild_id.member(ctx, msg.author.id).await?;

    let target_id = args
        .single::<UserId>()
        .map_err(|_| FabiError::InvalidArgument)?;
    let target = guild_id
        .member(&ctx.http, target_id)
        .await
        .map_err(|_| FabiError::MissingMember)?;

    let goulag_role = get_role(ctx, guild_id, "Fabien du goulag").await?;

    if has_role(ctx, &sender, &goulag_role).await {
        msg.channel_id.say(&ctx.http, "Salo, t'es au Goulag...").await?;
        return Ok(());
    }

    if !has_role(ctx, &target, &goulag_role).await {
        msg.channel_id.say(&ctx.http, "Vous ne pouvez pas cracher sur un honnête Fabien !").await?;
        return Ok(());
    }

    msg.channel_id.say(&ctx.http, format!("{} crache sur {} !", sender, target)).await?;

    Ok(())
}
