use serenity::{
    prelude::*,
    model:: prelude::*,
    framework::standard::{
        CommandResult, 
        macros::command,
    },
};

#[command]
#[only_in(guilds)]
#[description = "Quelques petites infos sur le serveur"]
pub async fn server_info(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let guild = match guild_id.to_guild_cached(&ctx) {
        Some(g) => g,
        None => return Ok(()),
    };

    let create_date = guild_id
        .created_at()
        .format("%d/%m/%Y %H:%M:%S")
        .to_string();

    let icon = match guild.icon_url() {
        Some(i) => i,
        None => "https://i.imgur.com/zPKzLoe.gif".to_string(),
    };

    let desc = match guild.description {
        Some(ref d) => d,
        None => "",
    };

    let guild_name = &guild.name;

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(guild_name);
            e.description(desc);
            e.thumbnail(icon);

            e.field("Date de création", create_date, false);
            e.field("Nombre de membres", guild.member_count.to_string(), false);

            e
        });

        m
    }).await?;

    Ok(()) 
}
