mod commands;
mod illegal_message;
pub mod utils;

use std::{
    collections::HashSet,
    env,
};

use serenity::{
    async_trait,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    http::Http,
    model::{
        event::ResumedEvent,
        gateway::Ready,
        channel::Message,
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
};

use illegal_message::illegal_message;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().contains("bertrand") {
            illegal_message(&ctx, &msg).await;
        }
    }
}

#[group]
#[commands(server_info, give_me_back_my_role, clear_chan, goulag, nogoulag, spit_on)]
struct General;

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
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}
