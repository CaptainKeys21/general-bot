use serenity::{builder::CreateEmbed, model::user::User};

use super::constants::{COLOR_FAIL, COLOR_WARN, COLOR_OKAY};

pub fn build_fail_embed(author: &User, err: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_FAIL);
    embed.title("Erro ao executar:");
    embed.description(err);
    embed.footer(|f| f.text(format!("Executado por: {}", author.tag())));
    embed
}

pub fn build_warn_embed(author: &User, warn: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_WARN);
    embed.title("Impedido de executar:");
    embed.description(warn);
    embed.footer(|f| f.text(format!("Executado por: {}", author.tag())));
    embed
}

pub fn build_success_embed(author: &User, warn: &str) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.color(COLOR_OKAY);
    embed.title("Impedido de executar:");
    embed.description(warn);
    embed.footer(|f| f.text(format!("Executado por: {}", author.tag())));
    embed
}