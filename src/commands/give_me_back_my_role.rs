use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        CommandResult, 
        macros::command,
    },
};

use crate::utils::*;

#[command]
pub async fn give_me_back_my_role(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 86385760969699328 {
        msg.channel_id.say(&ctx.http, "Non.").await?;
        return Ok(());
    }

    if let Ok(mut member) = get_message_member(ctx, msg).await {
        if let Ok(role) = get_role(ctx, msg.guild_id.unwrap(), "Fabien le Président").await {
            member.add_role(&ctx.http, role).await?;
        }
    }

    Ok(())
}
