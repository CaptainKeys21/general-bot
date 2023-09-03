mod commands;
mod services;
mod cache;
mod events;
mod utils;
mod models;
mod api;

use api::router::{build_router, RouterData};
use models::configs::general::GeneralConfig;
use poise::PrefixFrameworkOptions;
use serenity::{
    http::Http,
    prelude::GatewayIntents,
};
use utils::setup::{dynamic_prefix, get_owners};



use std::{
    net::SocketAddr, 
    sync::Arc
};

use std::{
    error::Error,
    env,
};

use crate::models::configs::config_manager::ConfigManager;
use crate::services::mongodb::Mongodb;
use crate::events::{
    checkers::command_check,
    hooks::{
        before::pre_command,
        after::post_command,
        dispatch_error::on_error,
    }
};
use crate::commands::{
    general::ping::ping,
    admin::admin,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Database setup
    let db_uri = env::var("DATABASE_URI").expect("Expect database URI");
    let database = Mongodb::new(db_uri).await?; 

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
    let owners = get_owners(&http).await;

    // All intents because fuck it why not?
    let intents = GatewayIntents::all();

    let prefix_options = PrefixFrameworkOptions {
        dynamic_prefix: Some(dynamic_prefix),
        ..Default::default()
    };

    //Create Handler
    let mut handler = events::Handler {
      options: poise::FrameworkOptions {
        commands: vec![ping(), admin()],
        on_error,
        pre_command,
        post_command,
        owners,
        prefix_options,
        command_check: Some(command_check),
        ..Default::default()
      }  
    };
    poise::set_qualified_names(&mut handler.options.commands);
    
    // Cliente builder
    let mut client = serenity::Client::builder(token, intents)
        .event_handler_arc(Arc::new(handler))
        .application_id(app_id.parse::<u64>()?)
        .cache_settings(| cfg | cfg.max_messages(100))
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
        let router_data = RouterData {
            bot_data: client_data.clone(),
        };

        // build our application with a route
        let app = build_router(router_data);

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
