use mongodb::bson::{doc, DateTime};
use serenity::model::prelude::Message;
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
            "type": level.to_string(),
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

    pub async fn command(&self, level: LogType, cmd_name: &str, msg: &Message, extra_msg: &str, print_line: bool) {
        let mut command_args: Vec<&str> = msg.content.split_whitespace().collect();
        command_args.remove(0); //* Remove the command name from the args vector
        
        let command_author = &msg.author;

        let date_now = Utc::now();

        let data = doc! {
            "type": level.to_string(),
            "author": {
                "id": command_author.id.0 as u32,
                "tag": command_author.tag(),
            },
            "command": {
                "name": cmd_name,
                "args": command_args,
            },
            "message": extra_msg,
            "time": DateTime::from_chrono(date_now),
        };

        if print_line {
            println!("{} | [{} | {cmd_name}] => {} | {extra_msg}", date_now.timestamp(), level.to_string(), command_author.tag());
        }

        if let Err(e) = self.database.insert_one("Logger", "commands", data).await {
            panic!("{}", e)
        }
    }
}