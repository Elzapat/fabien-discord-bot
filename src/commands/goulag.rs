use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        Args, CommandResult, 
        macros::command,
    },
};

use crate::utils::*;

#[command]
pub async fn goulag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the ID of the guild
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "Message not send in a guild".to_string()).await?;
            return Ok(());
        },
    };

    // Get the sender member
    let sender = match get_message_member(ctx, msg).await {
        Ok(m) => m,
        Err(e) => {
            msg.channel_id.say(&ctx.http, &format!("Erreur : {}", e)).await?;
            return Ok(());
        },
    };

    // Getting the target user of the command
    let mut target = match args.single::<UserId>() {
        Ok(user_id) => match guild_id.member(&ctx.http, user_id).await {
            Ok(m) => m,
            Err(e) => {
                msg.channel_id.say(&ctx.http, format!("Cible introuvable ({})", e)).await?;
                return Ok(());
            },
        },
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Usage : ?goulag @personne_pas_gentille ({})", e)).await?;
            return Ok(());
        },
    };

    // Get the gulag role
    let gulag_role = match get_role(ctx, guild_id, "Fabien du goulag").await {
        Ok(r) => r,
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Erreur : {}", e)).await?;
            return Ok(());
        },
    };

    // Check if the sender is in in the gulag
    if has_role(&ctx, &sender, &gulag_role).await {
        msg.channel_id.say(&ctx.http, "Bah non en fait".to_string()).await?;
        return Ok(());
    }

    // Check if the target is already in the gulag
    if has_role(&ctx, &target, &gulag_role).await {
        msg.channel_id.say(&ctx.http, "La cible est déjà au Goulag !".to_string()).await?;
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
    if let Err(e) = give_only_role(ctx, &mut target, &gulag_role).await {
        msg.channel_id.say(&ctx.http, format!("Erreur : {}", e)).await?;
        return Ok(());
    }

    msg.channel_id.say(&ctx.http, format!("{} a bien été emmené au Goulag", target)).await?;

    Ok(())
}
