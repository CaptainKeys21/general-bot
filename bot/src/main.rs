mod commands;
mod services;
mod cache;
mod events;
mod utils;
mod models;
mod api;

use api::router::build_router;
use cache::ConfigManagerCache;
use models::configs::general::GeneralConfig;
use serenity::{
    framework::{
        StandardFramework,
    },
    http::Http,
    prelude::GatewayIntents,
};


use std::net::SocketAddr;

use std::{
    collections::HashSet,
    error::Error,
    env,
};

use crate::models::configs::config_manager::ConfigManager;
use crate::services::mongodb::Mongodb;
use crate::events::hooks;
use crate::commands::{
    GENERAL_GROUP,
    ADMIN_GROUP
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = Mongodb::new().await; // Database setup
    // if let Err(e) = dotenv::from_filename("./config/bot.env") { // Enviroment variables setup
    //     println!("Env file not found: {}", e);
    // }

    let c_manager = ConfigManager::new(database.clone()).await;

    // Retrieving token and app_id, if get a error, try to get in the enviroment variables
    let data = c_manager.get_many::<GeneralConfig>(Some(&["token", "app_id",])).await;

    let token = match data.get("token") {
        Some(d) => d.data.clone(),
        None => {
            println!("bot token not found in database, trying enviroment variables");
            env::var("BOT_TOKEN").expect("Expect bot token from enviroment variable")
        },
    };
    let app_id = match data.get("app_id") {
        Some(d) => d.data.clone(),
        None => {
            println!("application id not found in database, trying enviroment variables");
            env::var("APPLICATION_ID").expect("Expect bot token from enviroment variable")
        },
    };

    // HTTP client to make request to the discord api
    let http = Http::new(&token);

    // Getting application owners
    let owners = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            owners.insert(info.owner.id);

            if let Some(team) = info.team {
                for member in &team.members {
                    owners.insert(member.user.id);
                }
            }

            owners
        }
        Err(why) => {
            println!("Erro ao acessar informações do bot: {}", why);
            HashSet::new()
        }
    };

    //Framework build
    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners);
            c.prefix("!gen>");
            c.dynamic_prefix(|ctx, _| {
                Box::pin(async move { 
                    let data = ctx.data.read().await;
                    let cfg_manager = data.get::<ConfigManagerCache>()?.read().await;
                    let prefix = cfg_manager.get_one::<GeneralConfig>("prefix").await;

                    match prefix {
                        Ok(d) => Some(d.data),
                        Err(_) => None
                    }
                })
            });

            c
        })
        .before(hooks::before::before)
        .after(hooks::after::after)
        .on_dispatch_error(hooks::dispatch_error::dispatch_error)
        .group(&GENERAL_GROUP)
        .group(&ADMIN_GROUP)
        .bucket("nospam", |b| b.delay(3).time_span(10).limit(3))
        .await;

    // All intents because fuck it why not?
    let intents = GatewayIntents::all();

    // Cliente builder
    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .event_handler(events::Handler)
        .application_id(app_id.parse::<u64>()?)
        .await?;

    let client_data = client.data.clone();
    // Storing some data in memory
    cache::fill(
        client_data.clone(), 
        c_manager,
        client.shard_manager.clone(),
        database
    ).await?;

    tokio::task::spawn(async move {
        // build our application with a route
        let app = build_router(client_data.clone());

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        if let Err(why) = axum::Server::bind(&addr).serve(app.into_make_service()).await {
            println!("Application api error: {}", why);
        };
    });

    // Start Client
    if let Err(why) = client.start_autosharded().await {
        println!("Client start error: {}", why);
    }

    Ok(())
}
