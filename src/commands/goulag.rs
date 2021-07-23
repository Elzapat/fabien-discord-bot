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
pub async fn goulag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // const USAGE: &str = "Usage: ?goulag @personne_pas_gentille";
    // Get the ID of the guild
    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;
    // Get the sender member
    let sender = guild_id.member(&ctx.http, msg.author.id).await?;

    // Getting the target user of the command
    let target_id = args
        .single::<UserId>()
        .map_err(|_| FabiError::InvalidArgument)?;
    let mut target = guild_id
        .member(&ctx.http, target_id)
        .await
        .map_err(|_| FabiError::MissingMember)?;

    // Get the gulag role
    let gulag_role = get_role(ctx, guild_id, "Fabien du goulag").await?;

    // Check if the sender is in in the gulag
    if has_role(&ctx, &sender, &gulag_role).await {
        msg.channel_id.say(&ctx.http, "Bah non en fait").await?;
        return Ok(());
    }

    // Check if the target is already in the gulag
    if has_role(&ctx, &target, &gulag_role).await {
        msg.channel_id.say(&ctx.http, "La cible est déjà au Goulag !").await?;
        return Ok(())
    }

    // Check if the sender has the required permissions
    if let Ok(perms) = sender.permissions(&ctx).await {
        if !perms.manage_messages() {
            msg.channel_id.say(&ctx.http, "Vous n'avez pas les permissions pour faire ça").await?;
            return Ok(());
        }
    }

    // Remove every role and give the gulag role to the target
    give_only_role(ctx, &mut target, &gulag_role).await?;

    msg.channel_id
        .say(&ctx.http, format!("{} a bien été emmené au Goulag", target))
        .await?;

    Ok(())
}
