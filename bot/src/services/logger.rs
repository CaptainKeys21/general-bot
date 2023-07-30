extern crate pretty_env_logger;
extern crate log;

use bson::Document;
use serde::Serialize;
use crate::utils::gb_serializer::Serializer;
use chrono::Utc;
use tokio::task;

use crate::services::mongodb::Mongodb;

use mongodb::bson::{
    doc, 
    Bson,
    DateTime, 
};

use poise::{Context, Command, CommandParameter};
use serenity::Error;

use serenity::model::{
    prelude::{
        Message, 
        PartialMember
    }, 
    user::User,
};

use std::error::Error as StdError;
use std::{
    fmt::{
        Display, 
        Formatter,
        Result as FmtResult
    }, 
    sync::Arc
};

#[allow(dead_code)]
pub enum LogType {
    Info,
    Waring,
    Error,
    Debug,
}

impl Display for LogType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            LogType::Info => write!(f, "Info"),
            LogType::Waring => write!(f, "Warning"),
            LogType::Error => write!(f, "Error"),
            LogType::Debug => write!(f, "Debug"),
        }
    }
}

pub enum MsgUpdateLog {
    Edited,
    Deleted,
}

pub struct Logger {
    database: Arc<Mongodb>,
    blocked_ids: Vec<u64>
}

impl Logger {
    pub fn new(database: Mongodb) -> Logger {
        pretty_env_logger::init();
        let database = Arc::new(database);
        
        Logger { 
            database,
            blocked_ids: Vec::new(), 
        }
    }

    pub fn default(&self, level: LogType, msg: &str) {
        let date_now = Utc::now();

        let data = doc! {
            "logType": level.to_string(),
            "message": msg,
            "time": DateTime::from_chrono(date_now),
        };

        match &level {
            LogType::Info => log::info!("{}", msg),
            LogType::Waring => log::warn!("{}", msg),
            LogType::Error => log::error!("{}", msg),
            LogType::Debug => log::debug!("{}", msg),
        }

        let task_database = Arc::clone(&self.database);
        task::spawn(async move {
            if let Err(e) = task_database.insert_one("Logger", "default", data).await {
                log::error!("{}", e);
            };
        });
    }

    pub fn message(&self, level: LogType, msg: &Message) {
        if self.check_blocked_ids(&msg.author, &msg.member) {
            return;
        }

        let log_message = format!("Message ({}) | {} => {}",
            msg.channel_id.0,
            msg.author.tag(),
            msg.content,
        );

        match &level {
            LogType::Info => log::info!("{}", log_message),
            LogType::Waring => log::warn!("{}", log_message),
            LogType::Error => log::error!("{}", log_message),
            LogType::Debug => log::debug!("{}", log_message),
        }

        let message_log = match msg.serialize(Serializer::new()) {
            Ok(b) => b,
            Err(why) => {
                log::error!("[Logger] => bson error: {}", why);
                return;
            }
        };
        let date_now = Utc::now();
        let data = doc! {
            "logType": level.to_string(),
            "data": message_log,
            "time": DateTime::from_chrono(date_now),
            "editedAt": Bson::Null,
            "deletedAt": Bson::Null,
        };

        let task_database = Arc::clone(&self.database);
        task::spawn(async move {
            if let Err(e) = task_database.insert_one("Logger", "messages", data).await {
                log::error!("{}", e);
            };
        });
    }

    pub fn update_message_log(&self, old_id: u64, operation: MsgUpdateLog) {
        let query = doc! { "data.id": old_id.to_string() };
        let date_now = Utc::now();
        let update = match operation {
            MsgUpdateLog::Edited => doc! { "$set": {"editedAt": DateTime::from_chrono(date_now)} },
            MsgUpdateLog::Deleted => doc! { "$set": {"deletedAt": DateTime::from_chrono(date_now)} },
        };
        
        let task_database = Arc::clone(&self.database);

        task::spawn(async move {
            match operation {
                MsgUpdateLog::Edited => {
                    if let Err(e) = task_database.update_one::<Document>("Logger", "messages", query, update).await {
                        log::error!("{}", e);
                    };
                    
                },
                MsgUpdateLog::Deleted => {
                    if let Err(e) = task_database.update_many("Logger", "messages", query, update).await {
                        log::error!("{}", e);
                    }
                }
            };
        });
    }

    pub fn command(&self, level: LogType, ctx: &Context<'_, (), Error>, extra_msg: Option<&str>) {
        let date_now = Utc::now();

        {
            let log_message = format!("Command ({}) | {} => {:?}",
                ctx.channel_id(),
                ctx.author().tag(),
                ctx.command(),
            );

            match &level {
                LogType::Info => log::info!("{}", log_message),
                LogType::Waring => log::warn!("{}", log_message),
                LogType::Error => log::error!("{}", log_message),
                LogType::Debug => log::debug!("{}", log_message),
            }
        }

        let author_id = match ctx.author().id.serialize(Serializer::new()) {
            Ok(d) => d,
            Err(_) => Bson::String("SERIALIZER_ERROR".to_string())
        };

        let command_bson = match self.serialise_poise_command(ctx.command()) {
            Ok(d) => d,
            Err(_) => Bson::String("SERIALIZER_ERROR".to_string())
        };

        let doc_log = doc! {
            "logType": level.to_string(),
            "author": author_id,
            "command": command_bson,
            "message": extra_msg,
            "time": DateTime::from_chrono(date_now),
        };

        let task_database = Arc::clone(&self.database);
        task::spawn(async move {
            if let Err(e) = task_database.insert_one("Logger", "commands", doc_log).await {
                log::error!("{}", e);
            };
        });
    }

    fn serialise_poise_command(&self, cmd: &Command<(), Error>) -> Result<Bson, Box<dyn StdError>> {
        let mut subcommand_doc: Vec<Bson> = Vec::new();
        for scmd in &cmd.subcommands {
            subcommand_doc.push(self.serialise_poise_command(scmd)?);
        }

        let mut parms_doc: Vec<Bson> = Vec::new();
        for param in &cmd.parameters {
            parms_doc.push(self.serialize_poise_command_parameter(param)?);
        }

        let cmd_doc = doc! {
            "subcommands": subcommand_doc,
            "names": {
                "main_name": &cmd.name,
                "name_localizations": cmd.name_localizations.serialize(Serializer::new())?,
                "qualified_name": &cmd.qualified_name,
                "identifying_name": &cmd.identifying_name,
                "aliases": cmd.aliases,
            },
            "parameters": parms_doc,
            "category": cmd.category,
            "descriptions": {
                "main_description": &cmd.description,
                "description_localizations": cmd.description_localizations.serialize(Serializer::new())?,
            },
            "permissions": {
                "default_member_permissions": cmd.default_member_permissions.serialize(Serializer::new())?,
                "required_permissions": cmd.required_permissions.serialize(Serializer::new())?,
                "required_bot_permissions": cmd.required_bot_permissions.serialize(Serializer::new())?,
            },
            "configs": {
                "hide_in_help": cmd.hide_in_help,
                "reuse_response": cmd.reuse_response,
                "owners_only": cmd.owners_only,
                "guild_only": cmd.guild_only,
                "dm_only": cmd.dm_only,
                "nsfw_only": cmd.nsfw_only,
                "invoke_on_edit": cmd.invoke_on_edit,
                "track_deletion": cmd.track_deletion,
                "broadcast_typing": cmd.broadcast_typing,
                "ephemeral": cmd.ephemeral,
            },
        };

        Ok(Bson::Document(cmd_doc))
    }

    fn serialize_poise_command_parameter(&self, cmd_param: &CommandParameter<(), Error>) -> Result<Bson, Box<dyn StdError>> {
        let mut choices_doc: Vec<Document> = Vec::new();
        for choice in &cmd_param.choices {
            choices_doc.push(doc! {
                "name": &choice.name,
                "localizations": choice.localizations.serialize(Serializer::new())?,
            });
        }

        let doc_param = doc! { 
            "names": {
                "main_name": &cmd_param.name,
                "name_localizations": cmd_param.name_localizations.serialize(Serializer::new())?,
            },
            "descriptions": {
                "main_description": &cmd_param.description,
                "description_localizations": cmd_param.description_localizations.serialize(Serializer::new())?,
            },
            "required": cmd_param.required,
            "channel_types": cmd_param.channel_types.serialize(Serializer::new())?,
            "choices": choices_doc,
        };

        Ok(Bson::Document(doc_param))
    }

    pub fn update_blocklist(&mut self, ids: Vec<u64>) {
        self.blocked_ids.extend(ids.iter());
        self.blocked_ids.sort();
        self.blocked_ids.dedup();
    }

    fn check_blocked_ids(&self, user: &User, member: &Option<PartialMember>) -> bool {
        let mut ids: Vec<u64> = Vec::new();

        ids.push(user.id.0);
  
        if let Some(m) = member {
            for role_id in &m.roles {
                ids.push(role_id.0);
            }
        };

        for id in &self.blocked_ids {
            if ids.contains(id) {
                return true;
            }
        }

        false
    }
}