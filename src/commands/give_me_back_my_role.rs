use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        CommandResult, 
        macros::command,
    },
};

use crate::utils::*;
use crate::fabi_error::FabiError;

#[command]
pub async fn give_me_back_my_role(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id != 86385760969699328 {
        msg.channel_id.say(&ctx.http, "Non.").await?;
        return Ok(());
    }

    let guild_id = msg.guild_id.ok_or(FabiError::NotInAGuild)?;
    let mut member = guild_id.member(&ctx.http, msg.author.id).await?;
    let role = get_role(ctx, guild_id, "Fabien le Président").await?;
    member.add_role(&ctx.http, role).await?;

    Ok(())
}
