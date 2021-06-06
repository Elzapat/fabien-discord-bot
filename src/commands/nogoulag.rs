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
pub async fn nogoulag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the guild
    let guild = match msg.guild_id {
        Some(id) => match id.to_guild_cached(&ctx).await {
            Some(g) => g,
            None => {
                msg.channel_id.say(&ctx.http, "Message not sent in a guild").await?;
                return Ok(());
            }
        }
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

    // Get the target of the command
    let mut target = match args.single::<UserId>() {
        Ok(user_id) => match guild.member(&ctx.http, user_id).await {
            Ok(m) => m,
            Err(e) => {
                msg.channel_id.say(&ctx.http, format!("Cible introuvable ({})", e)).await?;
                return Ok(());
            },
        },
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Usage : ?nogoulag @utilisateur, role ({})", e)).await?;
            return Ok(());
        },
    };

    // Get the wanted role for the target
    let wanted_role = match args.single::<String>() {
        Ok(r) => r,
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Usage : ?nogoulag @utilisateur, role ({})", e)).await?;
            return Ok(());
        },
    };

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

    if let Err(e) = give_only_role(ctx, &mut target, &target_role.unwrap()).await {
        msg.channel_id.say(&ctx.http, format!("Erreur : {}", e)).await?;
        return Ok(());
    }

    msg.channel_id
        .say(&ctx.http, format!("{} a été libéré et est maintenant un {}", target, wanted_role))
        .await?;

    Ok(())
}
