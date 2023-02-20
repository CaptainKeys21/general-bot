mod commands;
mod services;
mod cache;
mod events;

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

use crate::commands::{
    ping::*,
};

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = dotenv::dotenv() {
        println!("Erro ao encontrar arquivo .env: {}", e);
    }

    let token = env::var("BOT_TOKEN").expect("Esperado token do bot no arquivo .env");

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

    let prefix = env::var("BOT_PREFIX").expect("Expected bot prefix in .env file");
    let app_id = env::var("APPLICATION_ID").expect("Expected application id in .env file");

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&prefix))
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
    ).await?;

    if let Err(why) = client.start_autosharded().await {
        println!("Erro ao iniciar o cliente: {}", why)
    }

    Ok(())
}
