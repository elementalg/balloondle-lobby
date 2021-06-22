use std::process::Command;

use crate::core::matchmaking::match_server_details::MatchServerDetails;

pub trait MatchMaker {
    fn player_start_search(&self, name: &String, code: &u32, map: &String, gamemode: &String);
    fn player_stop_search(&self, name: &String, code: &u32);
    fn player_is_searching(&self, name: &String, code: &u32) -> bool;
    fn player_search_alive_ping(&self, name: &String, code: &u32) -> Option<(MatchServerDetails)>;

    fn player_start_playing_on_match(
        &self,
        player_name: &String,
        player_code: &u32,
        map: &String,
        gamemode: &String,
        match_code: &i64,
    ) -> bool;
    fn player_stop_playing_on_match(&self, name: &String, code: &u32) -> bool;

    fn match_register(
        &self,
        map: &String,
        gamemode: &String,
        gameserver_ip: &String,
        gameserver_port: &String,
    );
    fn match_start(
        &self,
        code: &i64,
        map: &String,
        gamemode: &String,
        gameserver_ip: &String,
        gameserver_port: &String,
        players: Option<String>,
    ) {
        println!(
            "Starting match: Code {}, Map {}, Gamemode {}",
            code, map, gamemode
        );

        Command::new("Balloondle")
            .env("PATH", "S:/Balloondle/Lobby/gameserver")
            .args(&[
                "-batchmode",
                "-nographics",
                "-map",
                map,
                "-gamemode",
                gamemode,
                "-code",
                code.to_string().as_str(),
                "-lobby",
                "http://localhost:8000",
                "-port",
                gameserver_port,
            ])
            .spawn();

        println!("Spawned process.");
    }

    fn get_available_match_details_for(
        &self,
        map: &String,
        gamemode: &String,
    ) -> Option<(i64, String, String)>;
    fn is_match_available(&self, map: &String, gamemode: &String) -> bool;
    fn is_any_match_starting(&self, map: &String, gamemode: &String) -> bool;

    fn server_ready(&self, map: &String, gamemode: &String, code: &i64) -> bool;
    fn server_stop(&self, map: &String, gamemode: &String, code: &i64) -> bool;
    fn players_stop_playing_on_match(
        &self,
        map: &String,
        gamemode: &String,
        match_code: &i64,
    ) -> bool;
}
