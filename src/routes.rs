use rand::Rng;
use rocket::response::content;
use crate::database::BalloondleDatabase;
use crate::core::auth::session_manager::SessionManager;
use crate::core::auth::player::Player;
use rocket::State;
use std::sync::{Mutex, Arc};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/auth/login?<code>&<name>")]
pub fn authenticate_login_player(session_manager: State<Arc<Mutex<SessionManager>>>, connection: BalloondleDatabase,
                                 name: String, code: u32) -> content::Json<String> {
    let result = connection.0.query("SELECT * FROM player WHERE name=$1 AND code=$2", &[&name, &code]).unwrap();

    if result.len() == 0 {
        content::Json(String::from("{ 'error': '0000#0001'}"))
    } else {
        let player: Player = Player::new(name.as_str(), code);

        let session_token: String = session_manager.get_mut().unwrap().create_session_for_player(player);

        content::Json(format!("{{ 'session_token': '{}' }}", session_token))
    }
}

#[post("/auth/signup?<name>")]
pub fn authenticate_signup_player(session_manager: State<Arc<Mutex<SessionManager>>>, connection: BalloondleDatabase,
                                  name: String) -> content::Json<String> {
    let code: u32 = try_signup_player(connection, name);

    let player: Player = Player::new(name.as_str(), code);

    let session_token: String = session_manager.get_mut().unwrap().create_session_for_player(player);

    content::Json(format!("{{ 'session_token': '{}' }}", session_token))
}

fn try_signup_player(connection: BalloondleDatabase, name: String) -> u32 {
    let generated_code: u32 = rand::prelude::thread_rng().gen_range(0u32..9999u32);

    let result = connection.0.query("SELECT * FROM player WHERE name=$1::text AND code=$2", &[&name, &generated_code]).unwrap();

    if result.len() == 0 {
        sign_up_player(connection, name, generated_code);

        generated_code
    } else {
        try_signup_player(connection, name)
    }
}

fn sign_up_player(connection: BalloondleDatabase, name: String, code: u32) {
    let result = connection.0.query("INSERT INTO player (name, code) VALUES ($1::text, $2)", &[&name, &code]).unwrap();

    println!("Affected rows on sign up: {}", result.len());
}