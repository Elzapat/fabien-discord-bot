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
#[description = "Faire sortir un Fabien du goulag en lui donnant le rôle demandé"]
pub async fn nogoulag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the guild
    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;
    let guild = guild_id.to_guild_cached(&ctx).await.ok_or(FabiError::NotInAGuild)?;

    // Get the sender member
    let sender = guild.member(ctx, msg.author.id).await?;

    // Get the target of the command
    let target_id = args
        .single::<UserId>()
        .map_err(|_| FabiError::InvalidArgument)?;
    let mut target = guild_id
        .member(&ctx.http, target_id)
        .await
        .map_err(|_| FabiError::MissingMember)?;

    // Get the wanted role for the target
    let wanted_role = args.single::<String>()?;

    // Check if the sender has the required permissions
    if let Ok(perms) = sender.permissions(&ctx).await {
        if !perms.manage_messages() {
            msg.channel_id.say(&ctx.http, "Vous n'avez pas les permissions pour faire ça.").await?;
            return Ok(());
        }
    }

    // Search the wanted role through all the guild roles
    // Doing this so the role name is not case sensitive
    let mut target_role = None;
    for (_, role) in guild.roles.iter() {
        if role.name.to_lowercase() == wanted_role.to_lowercase() {
            target_role = Some(role);
            break;
        }
    }

    if let None = target_role {
        msg.channel_id.say(&ctx.http, "Le role demandé est introuvable").await?;
        return Ok(());
    }

    give_only_role(ctx, &mut target, &target_role.unwrap()).await?;

    msg.channel_id
        .say(&ctx.http, format!("{} a été libéré et est maintenant un {}", target, wanted_role))
        .await?;

    Ok(())
}
