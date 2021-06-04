#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use crate::core::auth::session_manager::SessionManager;
use std::time::Duration;
use std::sync::{Arc, Mutex};

mod core;
mod routes;
mod database;
#[cfg(test)] mod test;
mod adapter;

const DEFAULT_SESSION_DURATION: Duration = Duration::from_secs(240u64);

fn main() {
    rocket::ignite().mount("/", routes![routes::index, routes::authenticate_login_player, routes::authenticate_signup_player])
        .attach(database::BalloondleDatabase::fairing())
        .launch();
}