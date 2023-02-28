use mongodb::bson::doc;
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

    pub async fn default(&self, level: LogType, msg: &str, print_line: bool) -> Result {
        let date_now = Utc::now();

        let data = doc! {
            "type": level.to_string(),
            "message": msg,
            "time": date_now.timestamp(),
        };

        if print_line {
            println!("{} | [{}] => {}", date_now.timestamp(), level.to_string(), msg);
        }

        if let Err(e) = self.database.insert_one("logger", "default", data).await {
            panic!("{}", e)
        }

        Ok(())
    }
}