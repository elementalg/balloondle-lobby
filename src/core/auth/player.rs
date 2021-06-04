use serde::{Serialize, Deserialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    name: String,
    code: u32,
}

impl Player {
    pub fn new(name: &str, code: u32) -> Player {
        Player {
            name: String::from(name),
            code
        }
    }
}