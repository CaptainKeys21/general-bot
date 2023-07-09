use std::collections::HashMap;

use serenity::builder::CreateEmbed;

use crate::utils::constants::COLOR_INFO;

pub fn create_help_embed(category_name: &str, category_help_text: Option<fn()->String>, command_infos: HashMap<String, Option<fn()->String>>) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed.title(category_name).color(COLOR_INFO);
    if let Some(txt) = category_help_text {
        embed.description(txt());
    } 

    for (key, text) in command_infos {
        if let Some(txt) = text {
            embed.field(key, txt(), false);
        }
    }

    embed
}