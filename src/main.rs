mod commands;
mod normal_messages;
pub mod utils;
pub mod fabi_error;

use std::{
    collections::HashSet,
    env,
};

use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::{
            macros::{ group, hook, help },
            help_commands,
            Args,
            CommandResult,
            HelpOptions,
            CommandGroup,
        },
    },
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        channel::Message,
        id::UserId,
    },
    prelude::*,
};

use commands::{
    server_info::*,
    give_me_back_my_role::*,
    clear_chan::*,
    goulag::*,
    nogoulag::*,
    spit_on::*,
    doomlag::*,
};

use normal_messages::check_illegal_message;

pub use fabi_error::{ FabiResult, FabiError };

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    async fn message(&self, _ctx: Context, _msg: Message) {
    }
}

#[group]
#[commands(goulag, nogoulag, doomlag)]
#[description = "Commandes pour faire sortir ou rentrer des Fabiens au Goulag"]
struct Goulag;

#[group]
#[commands(server_info, give_me_back_my_role, clear_chan)]
#[description = "Commandes utilitaires qui ne sont pas faites pour les Fabien lambdas"]
struct Utilitaires;

#[group]
#[commands(spit_on)]
struct Interactions;

#[help]
#[individual_command_tip = "Pour avoir plus d'infos sur une commande, mets-la en argument de cette commande !"]
async fn help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command {}", command_name),
        Err(e) => println!("Command '{}' return error {:?}", command_name, e),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(e) => panic!("Could not access application info: {:?}", e),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                .owners(owners)
                .prefix("?")
                .delimiters(vec![", ", ",", " ,"])
        )
        .after(after)
        .normal_message(check_illegal_message)
        .help(&HELP)
        .group(&INTERACTIONS_GROUP)
        .group(&GOULAG_GROUP)
        .group(&UTILITAIRES_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
