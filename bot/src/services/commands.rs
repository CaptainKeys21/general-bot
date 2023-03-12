use serenity::{
    builder::CreateApplicationCommand,
    client::Context,
    framework::standard::CommandResult,
    model::{
        application::{
            interaction::application_command::ApplicationCommandInteraction,
            command::{
                Command,
                CommandType
            }
        },
        guild::Guild,
    },
};

use crate::commands;

pub struct CommandManager {
    commands_registered: bool,
    commands: Vec<CreateApplicationCommand>,
}

impl CommandManager {
    pub fn new() -> Self {
        CommandManager { 
            commands_registered: false, 
            commands: CommandManager::build_commands(), 
        }
    }

    pub async fn on_command(
        &self,
        ctx: &Context,
        command: &ApplicationCommandInteraction,
    ) -> CommandResult {
        let command_name = command.data.name.to_lowercase();

        match command_name.as_str() {
            "ping" => commands::general::ping::slash_ping(ctx, command).await,
            "info" => commands::general::info::slash_info(ctx, command).await,
            e => {
                println!("Unknown application command received: {}", e);
                Ok(())
            }
        }
    }

    pub async fn register_commands_guild(&mut self, ctx: &Context, guild: &Guild) {
        match guild
            .set_application_commands(&ctx.http, |setter| {
                setter.set_application_commands(self.commands.clone())
            })
            .await
        {
            Err(e) => println!(
                "Unable to set application commands for guild '{}': {}",
                guild.id, e
            ),
            Ok(commands) => println!(
                "Registered {} commands in guild: {}",
                commands.len(),
                guild.id
            ),
        }
    }

    pub async fn register_commands_global(&mut self, ctx: &Context) {
        if self.commands_registered {
            return;
        }
        self.commands_registered = true;

        match Command::set_global_application_commands(&ctx.http, |setter| {
            setter.set_application_commands(self.commands.clone())
        }).await {
            Ok(cmds) => println!("Registered {} application commands", cmds.len()),
            Err(e) => println!("Unable to set application commands: {}", e),
        }
    }

    pub fn build_commands() -> Vec<CreateApplicationCommand> {
        let mut cmds = Vec::new();

        let mut cmd = CreateApplicationCommand::default();
        cmd.kind(CommandType::ChatInput)
            .name("ping")
            .description("Ping de teste");
        cmds.push(cmd);

        cmd = CreateApplicationCommand::default();
        cmd.kind(CommandType::ChatInput)
            .name("info")
            .description("Mostra informações do bot");
        cmds.push(cmd);

        cmds
    }
}