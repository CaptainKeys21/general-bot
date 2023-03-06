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

use std::{
    collections::HashSet,
    error::Error,
    env,
};

use crate::models::bot_config::BotConfig;
use crate::services::mongodb::Mongodb;
use crate::commands::{
    ping::*,
    info::*,
};

use crate::events::hooks;

//* General struct from Serenity
#[group]
#[commands(ping, info)]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = Mongodb::new().await; // Database setup
    if let Err(e) = dotenv::from_filename("./config/bot.env") { // Enviroment variables setup
        println!("Env file not found: {}", e);
    }

    // Retrieving token, app_id and prefix, if get a error, try to get in the enviroment variables
    let data = BotConfig::get_many(&database, &["token", "app_id", "prefix"]).await;

    let token = match data.get("token") {
        Some(d) => d.to_string(),
        None => {
            println!("bot token not found in database, trying enviroment variables");
            env::var("BOT_TOKEN").expect("Expect bot token from enviroment variable")
        },
    };
    let app_id = match data.get("app_id") {
        Some(d) => d.to_string(),
        None => {
            println!("application id not found in database, trying enviroment variables");
            env::var("APPLICATION_ID").expect("Expect bot token from enviroment variable")
        },
    };
    let prefix = match data.get("prefix") {
        Some(d) => d.to_string(),
        None => {
            println!("prefix not found in database, trying enviroment variables");
            env::var("BOT_PREFIX").expect("Expect bot token from enviroment variable")
        },
    };

    // HTTP client to make request to the discord api
    let http = Http::new(&token);

    //? Bot_id(ApplicationId) and app_id are two diferent things?
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

    // Framework build
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&prefix))
        .before(hooks::before::before)
        .after(hooks::after::after)
        .group(&GENERAL_GROUP)
        .bucket("nospam", |b| b.delay(3).time_span(10).limit(3))
        .await;

    // All intents because fuck it why not?
    let intents = GatewayIntents::all();

    // Cliente builder
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .application_id(app_id.parse::<u64>().unwrap())
        .await?;

    // Storing some data in memory
    cache::fill(
        client.data.clone(), 
        &prefix, 
        bot_id.0, 
        client.shard_manager.clone(),
        database
    ).await?;

    // Start Client
    if let Err(why) = client.start_autosharded().await {
        println!("Erro ao iniciar o cliente: {}", why)
    }

    Ok(())
}
