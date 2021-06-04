#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::sync::{Arc, Mutex};
use std::time::Duration;

mod core;
mod routes;
mod database;
#[cfg(test)]
mod test;
mod adapter;

const DEFAULT_SESSION_DURATION: Duration = Duration::from_secs(240u64);

fn main() {
    rocket::ignite().mount("/", routes![])
        .attach(database::Database::fairing())
        .launch();
}