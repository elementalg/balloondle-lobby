#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod core;
mod routes;
mod database;
#[cfg(test)]
mod test;
mod adapter;
mod error;

fn main() {
    rocket::ignite().mount("/", routes![routes::status, routes::user_startup])
        .attach(database::Database::fairing())
        .launch();
}