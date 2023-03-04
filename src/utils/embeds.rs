use serenity::{builder::{CreateEmbed, CreateMessage}, model::user::User};

use super::constants::COLOR_FAIL;

pub fn build_fail_embed(author: &User, err: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_FAIL);
    embed.title("Erro ao executar:");
    embed.description(err);
    embed.footer(|f| f.text(format!("Executado por: {}", author.tag())));
    embed
}