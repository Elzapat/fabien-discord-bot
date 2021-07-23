use serenity::{
    prelude::*,
    model::prelude::*,
};

use crate::utils::*;

pub async fn illegal_message(ctx: &Context, msg: &Message) {
    let _ = msg.react(&ctx.http, '😡').await;

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say(&ctx.http, "Message not sent in a guild".to_string()).await;
            return;
        },
    };

    if let Ok(mut member) = guild_id.member(ctx, msg.author.id).await {
        if let Ok(goulag_role) = get_role(ctx, guild_id, "Fabien du goulag").await {
            if let Err(e) = give_only_role(ctx, &mut member, &goulag_role).await {
                let _ = msg.channel_id.say(&ctx.http, format!("Erreur : {}", e)).await;
                return;                            
            }
        }
    }

    let _ = msg.channel_id 
        .say(&ctx.http, format!("Le criminel {} a été emmené au Goulag !", msg.author))
        .await;
}
