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
#[description = "Envoi d'un Fabien au Goulag"]
pub async fn goulag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // const USAGE: &str = "Usage: ?goulag @personne_pas_gentille";
    // Get the ID of the guild
    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;
    // Get the sender member
    let sender = guild_id.member(&ctx.http, msg.author.id).await?;

    // Get the gulag role
    let gulag_role = get_role(ctx, guild_id, "Fabien du goulag").await?;

    // Check if the sender is in the gulag
    if has_role(&ctx, &sender, &gulag_role).await {
        msg.channel_id.say(&ctx.http, "Bah non en fait").await?;
        return Ok(());
    }

    // Check if the sender has the required permissions
    if let Ok(perms) = sender.permissions(&ctx).await {
        if !perms.manage_messages() {
            msg.channel_id.say(&ctx.http, "Vous n'avez pas les permissions pour faire ça").await?;
            return Ok(());
        }
    }

    if let Ok(target_id) = args.single::<UserId>() {
        let mut target = guild_id
            .member(&ctx.http, target_id)
            .await
            .map_err(|_| FabiError::MissingMember)?;

        // Check if the target is already in the gulag
        if has_role(&ctx, &target, &gulag_role).await {
            msg.channel_id.say(&ctx.http, "La cible est déjà au Goulag !").await?;
            return Ok(())
        }

        // Remove every role and give the gulag role to the target
        give_only_role(ctx, &mut target, &gulag_role).await?;

        msg.channel_id
            .say(&ctx.http, format!("{} a bien été emmené au Goulag", target))
            .await?;

        Ok(())
    } else if let Ok(role_id) = args.single::<RoleId>() {
        let target_role = role_id
            .to_role_cached(&ctx.cache)
            .await
            .ok_or::<Box<dyn Error + Send + Sync>>(Box::new(FabiError::InvalidArgument).into())?;
        let mut members = guild_id.members(&ctx.http, None, None).await?;
        let mut goulaged_members = Vec::new();

        for member in members.iter_mut() {
            if has_role(&ctx, &member, &target_role).await {
                give_only_role(&ctx, member, &gulag_role).await?;
                goulaged_members.push(member.user.id);
            }
        }

        let one_member = goulaged_members.len() == 1;
        let mut message = String::new();
        for (i, user_id) in goulaged_members.iter().enumerate() {
            let last_member = i == goulaged_members.len() - 1;
            message.push_str(&format!("<@{}>{}", user_id, if last_member { " " } else { ", " }));
        }
        msg.channel_id.say(&ctx, format!(
            "{}{} été emmené{} au Goulag",
            message,
            if one_member { "a" } else { "ont" },
            if one_member { "" } else { "s" },
        )).await?;

        Ok(())
    } else {
        Err(Box::new(FabiError::InvalidArgument).into())
    }
}
