use rocket::http::Status;
use rocket::response::status::{Accepted, BadRequest, Created};

use crate::adapter::postgresql_matchmaker::PostgreSQLMatchMaker;
use crate::core::matchmaking::matchmaker::MatchMaker;
use crate::database::Database;

#[post("/matchmaking/search?<name>&<code>&<map>&<gamemode>")]
pub fn matchmaking_search(
    database: Database,
    name: String,
    code: u32,
    map: String,
    gamemode: String,
) -> Result<Created<String>, BadRequest<String>> {
    let matchmaker = get_matchmaker_implementation(database);

    // Return a bad request containing an error, because the player is already searching for
    // a match.
    if matchmaker.player_is_searching(&name, &code) {
        return Err(BadRequest(None));
    }

    matchmaker.player_start_search(&name, &code, &map, &gamemode);
    let created_resource_uri = String::from("");

    Ok(Created(created_resource_uri, None))
}

fn get_matchmaker_implementation(database: Database) -> Box<dyn MatchMaker> {
    Box::new(PostgreSQLMatchMaker::new(database))
}

#[post("/matchmaking/search_alive?<name>&<code>")]
pub fn matchmaking_alive(
    database: Database,
    name: String,
    code: u32,
) -> Result<Result<Accepted<String>, Status>, BadRequest<String>> {
    let matchmaker = get_matchmaker_implementation(database);

    if !matchmaker.player_is_searching(&name, &code) {
        return Err(BadRequest(None));
    }

    let match_details = matchmaker.player_search_alive_ping(&name, &code);

    match match_details {
        Some(match_details) => {
            let serialized_details = serde_json::to_string(&match_details);

            match serialized_details {
                Ok(serialized_details) => Ok(Ok(Accepted(Some(serialized_details)))),
                Err(_) => Err(BadRequest(Some(String::from(
                    "{ 'error': 'internal server error' }",
                )))),
            }
        }
        // Let the user know we received correctly its alive ping.
        None => Ok(Err(Status::Ok)),
    }
}

#[post("/matchmaking/stop_search?<name>&<code>")]
pub fn stop_matchmaking(
    database: Database,
    name: String,
    code: u32,
) -> Result<Accepted<String>, BadRequest<String>> {
    let matchmaker = get_matchmaker_implementation(database);

    if !matchmaker.player_is_searching(&name, &code) {
        return Err(BadRequest(None));
    }

    matchmaker.player_stop_search(&name, &code);
    Ok(Accepted(None))
}

#[post("/matchmaking/leave_match?<name>&<code>")]
pub fn leave_match(
    database: Database,
    name: String,
    code: u32,
) -> Result<Accepted<String>, BadRequest<String>> {
    let matchmaker = get_matchmaker_implementation(database);

    let result = matchmaker.player_stop_playing_on_match(&name, &code);

    match result {
        true => Ok(Accepted(None)),
        false => Err(BadRequest(None)),
    }
}

#[post("/matchmaking/server_ready?<map>&<gamemode>&<code>")]
pub fn server_ready(
    database: Database,
    map: String,
    gamemode: String,
    code: i64,
) -> Result<Accepted<String>, BadRequest<String>> {
    let matchmaker = get_matchmaker_implementation(database);

    let result = matchmaker.server_ready(&map, &gamemode, &code);

    match result {
        true => Ok(Accepted(None)),
        false => Err(BadRequest(None)),
    }
}

#[post("/matchmaking/server_stop?<map>&<gamemode>&<code>")]
pub fn server_stop(
    database: Database,
    map: String,
    gamemode: String,
    code: i64,
) -> Result<Accepted<String>, BadRequest<String>> {
    let matchmaker = get_matchmaker_implementation(database);

    let result = matchmaker.server_stop(&map, &gamemode, &code);

    match result {
        true => Ok(Accepted(None)),
        false => Err(BadRequest(None)),
    }
}
