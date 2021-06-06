use serenity::{
    prelude::*,
    model:: prelude::*,
};

pub async fn get_message_member(ctx: &Context, msg: &Message) -> Result<Member, String> {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Message not sent in a guild".to_string()),
    };

    match guild_id.member(&ctx.http, msg.author.id).await {
        Ok(m) => Ok(m),
        Err(_) => Err("Error getting the member".to_string()),
    }
}

pub async fn get_role(ctx: &Context, guild_id: GuildId, role: &str) -> Result<Role, String> {
    let guild = match guild_id.to_guild_cached(&ctx).await {
        Some(g) => g,
        None => return Err("Couldn't get guild from Guild ID".to_string()),
    };

    match guild.role_by_name(role) {
        Some(r) => Ok(r.clone()),
        None => Err(format!("Couldn't find the role {}", role)),
    }
}

pub async fn has_role(ctx: &Context, member: &Member, role: &Role) -> bool {
    let roles = match member.roles(&ctx).await {
        Some(r) => r,
        None => return false,
    };

    roles.contains(role)
}

pub async fn give_only_role(
    ctx: &Context,
    target: &mut Member,
    role: &Role
) -> Result<(), String> {
    let roles = match target.roles(&ctx.cache).await {
        Some(r) => r,
        None => vec![],
    };

    let mut role_ids: Vec<RoleId> = vec![];
    for role in roles.iter() {
        role_ids.push(role.id);
    }

    if let Err(e) = target.remove_roles(&ctx.http, &role_ids).await {
        return Err(e.to_string());
    }

    if let Err(e) = target.add_role(&ctx.http, role).await {
        return Err(e.to_string());
    }

    Ok(())
}
