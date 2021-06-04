use crate::core::auth::player::Player;
use std::time::Instant;

pub struct Session {
    player: Player,
    created_on: Instant,
}

impl Session {
    pub fn new(player: Player) -> Session {
        Session {
            player,
            created_on: Instant::now(),
        }
    }

    pub fn get_created_on(&self) -> &Instant {
        &self.created_on
    }

    pub fn expire(self) -> Player {
        self.player
    }
}