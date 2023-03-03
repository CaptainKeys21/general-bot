mod commands;
mod services;
mod cache;
mod events;
mod utils;
mod models;

use serenity::{
    framework::{
        standard::macros::group,
        StandardFramework,
    },
    http::Http,
    prelude::GatewayIntents,
};

use bson::doc;

use std::{
    collections::HashSet,
    error::Error,
    env,
};

use crate::services::mongodb::Mongodb;
use crate::commands::{
    ping::*,
    info::*,
};

#[group]
#[commands(ping, info)]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = Mongodb::new().await;
    if let Err(e) = dotenv::from_filename("./config/mongod.env") {
        println!("Erro ao encontrar arquivo .env: {}", e);
    }

    let query_filter = doc! {
        "$or": [
            {"name": "token"},
            {"name": "app_id"}, 
            {"name": "prefix"},
            ]
        };

    let (token, app_id, prefix) = match database.get("GeneralBot", "config", query_filter).await {
        Some(res) => {
            let mut token = String::new();
            let mut app_id = String::new();
            let mut prefix = String::new();

            for doc in res {
                match doc.get_str("name").expect("Expected config name from vector") {
                    "token"  => token = String::from(doc.get_str("data").expect("Expected data from vector")),
                    "app_id" => app_id = String::from(doc.get_str("data").expect("Expected data from vector")),
                    "prefix" => prefix = String::from(doc.get_str("data").expect("Expected data from vector")),
                    &_ => panic!("I dont know...")
                }
            }

            (token, app_id, prefix)
        }
        None => {
            println!("bot token not found in database, trying enviroment variables");
            let token = env::var("BOT_TOKEN").expect("Expect bot token from enviroment variable");
            let app_id = env::var("APPLICATION_ID").expect("Expect bot token from enviroment variable");
            let prefix = env::var("BOT_PREFIX").expect("Expect bot token from enviroment variable");

            (token, app_id, prefix)
        }
    };

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            owners.insert(info.owner.id);

            if let Some(team) = info.team {
                for member in &team.members {
                    owners.insert(member.user.id);
                }
            }

            (owners, info.id)
        }
        Err(why) => {
            println!("Erro ao acessar informações do bot: {}", why);
            println!("Tentando variavel de ambiente para id do bot...");
            let id = env::var("BOT_ID").expect("Não foi possível encontrar variavel de ambiente BOT_ID");
            let bot_id = id.parse::<u64>().expect("Id de bot inválida");
            (HashSet::new(), serenity::model::id::ApplicationId(bot_id))
        }
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&prefix))
        .before(events::before)
        .after(events::after)
        .group(&GENERAL_GROUP)
        .bucket("nospam", |b| b.delay(3).time_span(10).limit(3))
        .await;

    let intents = GatewayIntents::GUILDS 
        | GatewayIntents::MESSAGE_CONTENT 
        | GatewayIntents::GUILD_INTEGRATIONS 
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGES;

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .application_id(app_id.parse::<u64>().unwrap())
        .await?;

    cache::fill(
        client.data.clone(), 
        &prefix, 
        bot_id.0, 
        client.shard_manager.clone(),
        database
    ).await?;

    if let Err(why) = client.start_autosharded().await {
        println!("Erro ao iniciar o cliente: {}", why)
    }

    Ok(())
}
