use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        Args, CommandResult,
        macros::command,
    },
};
use std::error::Error;
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
    let target_id = args.single::<UserId>()?;
    let mut target = guild_id.member(&ctx.http, target_id).await?;

    // Get the wanted role for the target
    let role_id = args.single::<RoleId>()?;

    // Check if the sender has the required permissions
    if let Ok(perms) = sender.permissions(&ctx).await {
        if !perms.manage_messages() {
            msg.channel_id.say(&ctx.http, "Vous n'avez pas les permissions pour faire ça.").await?;
            return Ok(());
        }
    }

    // Search the wanted role through all the guild roles
    // Doing this so the role name is not case sensitive
    let target_role = role_id
        .to_role_cached(&ctx.cache)
        .await
        .ok_or::<Box<dyn Error + Send + Sync>>(Box::new(FabiError::InvalidArgument).into())?;

    give_only_role(ctx, &mut target, &target_role).await?;

    msg.channel_id
        .say(&ctx.http, format!("{} a été libéré et est maintenant un {}", target, target_role))
        .await?;

    Ok(())
}
