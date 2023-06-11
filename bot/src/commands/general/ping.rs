// use serenity::{
//     framework::standard::{
//         macros::command, 
//         Args, 
//         CommandResult
//     },
//     model::{
//         prelude::*,
//         application::interaction::{
//             InteractionResponseType::ChannelMessageWithSource,
//             application_command::ApplicationCommandInteraction,
//         }
//     },
//     prelude::*,
// };

use poise::{Context, command};
use serenity::Error;
use std::time::Instant;

#[command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_, (), Error>) -> Result<(), Error> {
    let old = Instant::now();
    let msg = ctx.say("Pong!\n...").await?;
    let new = Instant::now();

    msg.edit(ctx, |m| {
        m.content(format!("Pong!\n{} ms", (new - old).as_millis()))
    }).await?;

    Ok(())
}

// #[command]
// pub async fn ping(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
//     let old = Instant::now();
//     let mut m = msg.channel_id.say(&ctx.http, "Pong!\n...").await?;
//     let new = Instant::now();

//     m.edit(ctx, |m| {
//         m.content(format!("Pong!\n{} ms", (new - old).as_millis()))
//     }).await?;

//     Ok(())
// }

// pub mod slash {
//     use std::time::Instant;

//     use serenity::{
//         framework::standard::CommandResult,
//         model::{
//             application::interaction::{
//                 InteractionResponseType::ChannelMessageWithSource,
//                 application_command::ApplicationCommandInteraction,
//             }
//         },
//         prelude::*, 
//         builder::CreateApplicationCommand,
//     };

//     pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
//         command.name("ping").description("Testa a conexão com o bot")
//     }

//     pub async fn run(ctx: &Context, msg: &ApplicationCommandInteraction) -> CommandResult {
//         let old = Instant::now();
//         msg.create_interaction_response(&ctx.http, |resp| {
//             resp.kind(ChannelMessageWithSource)
//                 .interaction_response_data(|data| data.content("Pong!\n.."))
//         }).await?;
    
//         let new = Instant::now();
    
//         msg.edit_original_interaction_response(&ctx.http, |resp| {
//             resp.content(format!("🏓 Pong!\n{} ms", (new - old).as_millis()))
//         }).await?;
    
//         Ok(())
//     }
// }