use std::collections::HashMap;

use poise::{
    command,
    Context,
};

use serenity::Error;

use crate::utils::embeds::help_embeds::create_help_embed;

mod ban;
mod softban;
mod kick;
mod mute;
mod warnings;
mod note;

use self::{
    ban::{
        ban::ban,
        unban::unban,
    },
    mute::{
        mute::mute,
        unmute::unmute,
    },
    warnings::{
        warnings::warnings,
        warn::warn,
        delwarn::delwarn,
    },
    note::{
        note::note,
        notes::notes,
        delnote::delnote,
        editnote::editnote,
        clearnotes::clearnotes,
    },
    softban::softban,
    kick::kick,
};


fn help() -> String {
    String::from("\
    Comandos de administrador.\n
    [prefixo]admin [comando] [..argumentos]
    ")
}

#[command(
    prefix_command, 
    slash_command,
    help_text_fn="help",
    subcommands(
        "ban", "unban", 
        "softban", 
        "kick", 
        "mute", "unmute", 
        "warn", "delwarn", "warnings",
        "note", "notes", "delnote", "editnote", "clearnotes"
    ),
    aliases("adm")
)]
pub async fn admin(ctx: Context<'_, (), Error>, _arg: Option<String>) -> Result<(), Error> { 
    let mut command_infos = HashMap::new();
    for command in &ctx.command().subcommands {
        command_infos.insert(String::from(&command.name), command.help_text);
    }

    let embed = create_help_embed("admin", ctx.command().help_text, command_infos);

    ctx.send(|m| {
        m.embeds.push(embed);
        m
    }).await?;
    Ok(()) 
}