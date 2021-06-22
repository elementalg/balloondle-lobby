#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod adapter;
mod core;
mod database;
mod error;
mod routes;
#[cfg(test)]
mod test;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::auth::user_signup,
                routes::auth::user_login,
                routes::matchmaking::matchmaking_search,
                routes::matchmaking::matchmaking_alive,
                routes::matchmaking::leave_match,
                routes::matchmaking::stop_matchmaking,
                routes::matchmaking::server_ready,
                routes::matchmaking::server_stop
            ],
        )
        .attach(database::Database::fairing())
        .launch();
}
