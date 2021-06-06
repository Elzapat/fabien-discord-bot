use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        Args, CommandResult, 
        macros::command,
    },
};

#[command]
pub async fn clear_chan(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let n = args.single::<u64>()?;

    if let Ok(Channel::Guild(channel)) = msg.channel_id.to_channel(&ctx.http).await {
        if let Ok(mut last_messages) = channel.messages(&ctx.http, |r| {
            r.before(msg.id);
            r.limit(n);

            r
        }).await {
            last_messages.push(msg.clone());
            channel.delete_messages(&ctx.http, last_messages).await?;
        }
    }

    Ok(())
}
