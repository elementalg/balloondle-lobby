use std::time::SystemTime;

use rocket_contrib::databases::postgres::rows::Rows;
use rocket_contrib::databases::postgres::Error;

use crate::core::matchmaking::match_server_details::MatchServerDetails;
use crate::core::matchmaking::matchmaker::MatchMaker;
use crate::core::matchmaking::port_assigner::get_free_random_port_for_gameserver;
use crate::database::Database;

static GAMESERVER_IP_HOST: &str = "127.0.0.1";

pub struct PostgreSQLMatchMaker {
    database: Database,
}

impl PostgreSQLMatchMaker {
    pub fn new(database: Database) -> PostgreSQLMatchMaker {
        PostgreSQLMatchMaker { database }
    }
}

impl PostgreSQLMatchMaker {
    fn player_get_search_details(&self, name: &String, code: &u32) -> Option<(String, String)> {
        let sql_query = "SELECT map_name, gamemode_name FROM search_queue WHERE player_name=$1 AND player_code=$2";

        let unchecked_rows = self.database.0.query(sql_query, &[name, code]);

        match unchecked_rows {
            Ok(rows) => {
                let row = rows.get(0);
                let map_name = row.get(0);
                let gamemode_name = row.get(1);

                Some((map_name, gamemode_name))
            }
            Err(_) => None,
        }
    }
}

impl MatchMaker for PostgreSQLMatchMaker {
    fn player_start_search(&self, name: &String, code: &u32, map: &String, gamemode: &String) {
        println!("Start search.");
        let sql_query: &str = "INSERT INTO search_queue (player_name, player_code, map_name, gamemode_name) VALUES ($1, $2, $3, $4)";

        self.database
            .0
            .execute(sql_query, &[name, code, map, gamemode]);

        // Request a gameserver to get started if there's none available or starting already.
        if !self.is_match_available(map, gamemode) && !self.is_any_match_starting(map, gamemode) {
            let gameserver_ip: String = String::from(GAMESERVER_IP_HOST);
            let gameserver_port: String = get_free_random_port_for_gameserver().to_string();
            self.match_register(map, gamemode, &gameserver_ip, &gameserver_port);
        }
    }

    fn player_stop_search(&self, name: &String, code: &u32) {
        let sql_query: &str = "DELETE FROM search_queue WHERE player_name=$1 AND player_code=$2";

        self.database.0.execute(sql_query, &[name, code]);
    }

    fn player_is_searching(&self, name: &String, code: &u32) -> bool {
        let sql_query: &str =
            "SELECT count(*) FROM search_queue WHERE player_name=$1 AND player_code=$2";

        let unchecked_rows = self.database.0.query(sql_query, &[name, code]);

        match unchecked_rows {
            Ok(rows) => {
                if rows.len() == 1 {
                    let row = rows.get(0);

                    let searching: i64 = row.get(0);

                    if searching == 1 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    fn player_search_alive_ping(&self, name: &String, code: &u32) -> Option<(MatchServerDetails)> {
        let sql_query =
            "UPDATE search_queue SET alive_time=NOW() WHERE player_name=$1 AND player_code=$2";

        self.database.0.execute(sql_query, &[name, code]);

        let search_details = self.player_get_search_details(name, code);

        match search_details {
            Some(search_details) => {
                let map = String::from(&search_details.0);
                let gamemode = String::from(&search_details.1);

                // Pass the details of the available match, otherwise request a gameserver to start
                // only if there's not already one starting with the same map and gamemode.
                if self.is_match_available(&search_details.0, &search_details.1) {
                    let match_details =
                        self.get_available_match_details_for(&search_details.0, &search_details.1);

                    match match_details {
                        Some(match_details) => {
                            self.player_start_playing_on_match(
                                name,
                                code,
                                &search_details.0,
                                &search_details.1,
                                &match_details.0,
                            );

                            Some(MatchServerDetails::new(
                                map,
                                gamemode,
                                match_details.1,
                                match_details.2,
                            ))
                        }
                        None => None,
                    }
                } else {
                    if !self.is_any_match_starting(&search_details.0, &search_details.1) {
                        let gameserver_ip: String = String::from(GAMESERVER_IP_HOST);
                        let gameserver_port: String =
                            get_free_random_port_for_gameserver().to_string();
                        self.match_register(
                            &search_details.0,
                            &search_details.1,
                            &gameserver_ip,
                            &gameserver_port,
                        );

                        Some(MatchServerDetails::new(
                            map,
                            gamemode,
                            gameserver_ip,
                            gameserver_port,
                        ))
                    } else {
                        None
                    }
                }
            }
            None => None,
        }
    }

    fn player_start_playing_on_match(
        &self,
        player_name: &String,
        player_code: &u32,
        map: &String,
        gamemode: &String,
        match_code: &i64,
    ) -> bool {
        let sql_query = "INSERT INTO playing_match (player_name, player_code, match_map, match_gamemode, match_code) VALUES ($1, $2, $3, $4, $5)";

        let affected_rows = self.database.0.execute(
            sql_query,
            &[player_name, player_code, map, gamemode, match_code],
        );

        self.player_stop_search(player_name, player_code);

        match affected_rows {
            Ok(affected_rows) => true,
            Err(e) => {
                println!("Error: {}", e);

                false
            }
        }
    }

    fn player_stop_playing_on_match(&self, name: &String, code: &u32) -> bool {
        let sql_query = "DELETE FROM playing_match WHERE player_name=$1 AND player_code=$2";

        let affected_rows = self.database.0.execute(sql_query, &[name, code]);

        match affected_rows {
            Ok(affected_rows) => true,
            Err(e) => {
                println!("Error: {}", e);

                false
            }
        }
    }

    fn match_register(
        &self,
        map: &String,
        gamemode: &String,
        gameserver_ip: &String,
        gameserver_port: &String,
    ) {
        println!("Registering match: {} {}", map, gamemode);
        let sql_query = "SELECT CREATE_GAME_MATCH($1, $2, $3, $4)";

        let unchecked_rows = self
            .database
            .0
            .query(sql_query, &[map, gamemode, gameserver_ip, gameserver_port]);

        match unchecked_rows {
            Ok(rows) => {
                let row = rows.get(0usize);

                let code: i64 = row.get(0usize);
                println!("Code: {}", code);
                self.match_start(&code, map, gamemode, gameserver_ip, gameserver_port, None);
            }
            Err(e) => {
                println!("Error: {}", e);

                ()
            }
        }
    }

    fn get_available_match_details_for(
        &self,
        map: &String,
        gamemode: &String,
    ) -> Option<(i64, String, String)> {
        let sql_query = "SELECT code, server_ip, server_port FROM game_match WHERE map_name=$1 AND gamemode_name=$2 AND match_state=$3 LIMIT 1";
        let match_state = String::from("running");

        let unchecked_rows = self
            .database
            .0
            .query(sql_query, &[map, gamemode, &match_state]);

        match unchecked_rows {
            Ok(rows) => {
                let row = rows.get(0usize);

                let code: i64 = row.get(0);
                let ip: String = row.get(1);
                let port: String = row.get(2);

                Some((code, ip, port))
            }
            Err(_) => None,
        }
    }

    fn is_match_available(&self, map: &String, gamemode: &String) -> bool {
        println!("Is match available: {} {}", map, gamemode);

        let sql_query = "SELECT COUNT(*) FROM game_match WHERE map_name=$1 AND gamemode_name=$2 AND match_state=$3";
        let match_state = String::from("running");

        let unchecked_rows = self
            .database
            .0
            .query(sql_query, &[map, gamemode, &match_state]);

        match unchecked_rows {
            Ok(rows) => {
                let row = rows.get(0usize);

                let count: i64 = row.get(0usize);

                if count > 0 {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    fn is_any_match_starting(&self, map: &String, gamemode: &String) -> bool {
        let sql_query = "SELECT COUNT(*) FROM game_match WHERE map_name=$1 AND gamemode_name=$2 AND match_state=$3";
        let match_state = String::from("starting");

        let unchecked_rows = self
            .database
            .0
            .query(sql_query, &[map, gamemode, &match_state]);

        match unchecked_rows {
            Ok(rows) => {
                let row = rows.get(0usize);

                let count: i64 = row.get(0usize);

                if count > 0 {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    fn server_ready(&self, map: &String, gamemode: &String, code: &i64) -> bool {
        let sql_query = "UPDATE game_match SET match_state=$1 WHERE map_name=$2 AND gamemode_name=$3 AND code=$4";
        let running_match_state = String::from("running");

        let result = self
            .database
            .0
            .execute(sql_query, &[&running_match_state, map, gamemode, code]);

        match result {
            Ok(affected_rows) => {
                if affected_rows == 1 {
                    true
                } else {
                    println!("Rows affected: {}", affected_rows);

                    false
                }
            }
            Err(e) => {
                println!("Error: {}", e);

                false
            }
        }
    }

    fn server_stop(&self, map: &String, gamemode: &String, code: &i64) -> bool {
        let sql_query = "UPDATE game_match SET match_state=$1 WHERE map_name=$2 AND gamemode_name=$3 AND code=$4";
        let running_match_state = String::from("ended");

        let result = self
            .database
            .0
            .execute(sql_query, &[&running_match_state, map, gamemode, code]);

        self.players_stop_playing_on_match(map, gamemode, code);

        match result {
            Ok(affected_rows) => {
                if affected_rows == 1 {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    fn players_stop_playing_on_match(
        &self,
        map: &String,
        gamemode: &String,
        match_code: &i64,
    ) -> bool {
        let sql_query =
            "DELETE FROM playing_match WHERE match_map=$1 AND match_gamemode=$2 AND match_code=$3";

        let result = self
            .database
            .0
            .execute(sql_query, &[map, gamemode, match_code]);

        match result {
            Ok(affected_rows) => true,
            Err(_) => false,
        }
    }
}
