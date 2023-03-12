pub mod general;
pub mod admin;

use crate::events::checkers::role_check::ROLE_CHECK;

use serenity::framework::standard::macros::group;
use admin::insert_users::*;
use general::{
    info::*,
    ping::*,
};


#[group]
#[checks(Role)]
#[commands(ping, info)]
struct General;

#[group]
#[prefix = "admin"]
#[commands(insert_users)]
struct Admin;