use serde::{Deserialize, Serialize};

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    name: String,
    code: u32,
}

impl User {
    pub fn new(name: &str, code: u32) -> User {
        User {
            name: String::from(name),
            code,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_code(&self) -> &u32 {
        &self.code
    }
}