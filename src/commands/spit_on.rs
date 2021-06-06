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
pub async fn spit_on(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // Get the ID of the guild
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            msg.channel_id.say(&ctx.http, "Message not send in a guild".to_string()).await?;
            return Ok(());
        },
    };

    let sender = match get_message_member(ctx, msg).await {
        Ok(m) => m,
        Err(e) => {
            msg.channel_id.say(&ctx.http, &format!("Erreur : {}", e)).await?;
            return Ok(());
        },
    };

    let target = match args.single::<UserId>() {
        Ok(user) => match guild_id.member(&ctx.http, user).await {
            Ok(m) => m,
            Err(e) => {
                msg.channel_id.say(&ctx.http, format!("Cible introuvable ({})", e)).await?;
                return Ok(());
            },
        },
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Usage : ?spiton @personne_pas_gentille ({})", e)).await?;
            return Ok(());
        },
    };

    let goulag_role = match get_role(ctx, guild_id, "Fabien du goulag").await {
        Ok(r) => r,
        Err(e) => {
            msg.channel_id.say(&ctx.http, format!("Erreur : {}", e)).await?;
            return Ok(())
        },
    };

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
