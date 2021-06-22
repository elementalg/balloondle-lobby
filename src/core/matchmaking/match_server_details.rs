use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchServerDetails {
    map: String,
    gamemode: String,
    server_ip: String,
    server_port: String,
}

impl MatchServerDetails {
    pub fn new(
        map: String,
        gamemode: String,
        server_ip: String,
        server_port: String,
    ) -> MatchServerDetails {
        MatchServerDetails {
            map,
            gamemode,
            server_ip,
            server_port,
        }
    }
}
