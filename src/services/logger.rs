use mongodb::bson::{doc, DateTime, ser::Serializer};
use serde::{Serialize};
use serenity::model::{
    application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Message,
};
use std::fmt::{Display, Formatter, Result};
use chrono::Utc;
use crate::services::mongodb::Mongodb;

pub enum LogType {
    Info,
    Waring,
    Error,
    Debug,
}

impl Display for LogType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LogType::Info => write!(f, "Info"),
            LogType::Waring => write!(f, "Warning"),
            LogType::Error => write!(f, "Error"),
            LogType::Debug => write!(f, "Debug"),
        }
    }
}

pub enum CmdOrInt<'a> {
    Command(&'a Message),
    Interaction(&'a ApplicationCommandInteraction)
}

pub struct Logger {
    database: Mongodb,
}

impl Logger {
    pub fn new(database: Mongodb) -> Logger {
        Logger { database }
    }

    pub async fn default(&self, level: LogType, msg: &str, print_line: bool) {
        let date_now = Utc::now();

        let data = doc! {
            "logType": level.to_string(),
            "message": msg,
            "time": DateTime::from_chrono(date_now),
        };

        if print_line {
            println!("{} | [{}] => {}", date_now.timestamp(), level.to_string(), msg);
        }

        if let Err(e) = self.database.insert_one("Logger", "default", data).await {
            panic!("{}", e)
        }
    }

    pub async fn command(&self, level: LogType, cmd_name: &str, cmd_detail: CmdOrInt<'_>, extra_msg: Option<&str>) {

        let data = match cmd_detail {
            CmdOrInt::Command(msg) => {
                let mut command_args: Vec<&str> = msg.content.split_whitespace().collect();
                command_args.remove(0); //* Remove the command name from the args vector
            
                let command_author = msg.author.serialize(Serializer::new()).unwrap();

                let date_now = Utc::now();

                let data = doc! {
                    "logType": level.to_string(),
                    "author": command_author,
                    "command": {
                        "name": cmd_name,
                        "args": command_args,
                        "isSlashCommand": false,
                    },
                    "message": extra_msg,
                    "time": DateTime::from_chrono(date_now),
                };

                data
            }

            CmdOrInt::Interaction(int) => {
                let command_author = int.user.serialize(Serializer::new()).unwrap();

                let interaction_data = int.data.serialize(Serializer::new()).unwrap();

                let date_now = Utc::now();

                let data = doc! {
                    "logType": level.to_string(),
                    "author": command_author,
                    "command": interaction_data,
                    "message": extra_msg,
                    "time": DateTime::from_chrono(date_now),
                };

                data
            }
        };
        

        if let Err(e) = self.database.insert_one("Logger", "commands", data).await {}
    }
}