extern crate pretty_env_logger;
extern crate log;

use serde::Serialize;
use chrono::Utc;
use tokio::task;

use crate::services::mongodb::Mongodb;

use mongodb::bson::{
    doc, 
    DateTime, 
    ser::Serializer

};

use serenity::model::{
    application::interaction::application_command::ApplicationCommandInteraction,
    prelude::{
        Message, 
        interaction::application_command::CommandDataOption
    },
};

use std::{
    fmt::{
        Display, 
        Formatter, 
        Result
    }, 
    sync::Arc
};

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
    database: Arc<Mongodb>,
}

impl Logger {
    pub fn new(database: Mongodb) -> Logger {
        pretty_env_logger::init();

        Logger { database: Arc::new(database) }
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

    pub fn command(&self, level: LogType, cmd_name: &str, cmd_detail: CmdOrInt<'_>, extra_msg: Option<&str>) {
        let data = match cmd_detail {
            CmdOrInt::Command(msg) => {
                let mut command_args: Vec<&str> = msg.content.split_whitespace().collect();
                command_args.remove(0); //* Remove the command name from the args vector

                {
                    let log_message = format!("prefix command | {} => [{} {}] {}", 
                        msg.author.tag(), 
                        cmd_name, 
                        command_args.join(" "), 
                        match extra_msg {
                            Some(m) => String::from("=> ") + m,
                            None => String::new(),
                        }
                    );

                    match &level {
                        LogType::Info => log::info!("{}", log_message),
                        LogType::Waring => log::warn!("{}", log_message),
                        LogType::Error => log::error!("{}", log_message),
                        LogType::Debug => log::debug!("{}", log_message),
                    }
                }
            
                let command_author = msg.author.serialize(Serializer::new()).unwrap();

                let date_now = Utc::now();

                let data = doc! {
                    "logType": level.to_string(),
                    "author": command_author,
                    "command": {
                        "name": cmd_name,
                        "args": command_args,
                    },
                    "message": extra_msg,
                    "time": DateTime::from_chrono(date_now),
                };

                data
            }

            CmdOrInt::Interaction(int) => {
                {
                    let log_message = format!("prefix command | {} => [{} | options: {}] => {}", 
                        int.user.tag(), 
                        cmd_name, 
                        &self.format_interaction_options(&int.data.options), 
                        match extra_msg {
                            Some(m) => String::from("=> ") + m,
                            None => String::new(),
                        }
                    );

                    match &level {
                        LogType::Info => log::info!("{}", log_message),
                        LogType::Waring => log::warn!("{}", log_message),
                        LogType::Error => log::error!("{}", log_message),
                        LogType::Debug => log::debug!("{}", log_message),
                    }
                }

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

        let task_database = Arc::clone(&self.database);
        task::spawn(async move {
            if let Err(e) = task_database.insert_one("Logger", "commands", data).await {
                log::error!("{}", e);
            };
        });
    }

    fn format_interaction_options(&self, options: &Vec<CommandDataOption>) -> String {
        let mut return_string = String::new();
        for option in options {

            let str_value = match &option.value {
                Some(val) => val.as_str().unwrap_or("!{EMPTY}"),
                None => "!{EMPTY}",
            };

            let mut option_string = String::from("(");
            option_string = option_string + &option.name + ": " + str_value;

            

            if !option.options.is_empty() {
                option_string = option_string + ", nested options: ";
                option_string = option_string + &self.format_interaction_options(&option.options);
            }

            return_string = return_string + &option_string;
        }

        return_string
    }
}