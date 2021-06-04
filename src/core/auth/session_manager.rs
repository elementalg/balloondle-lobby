use std::time::{Instant, Duration};
use std::collections::HashMap;
use crate::core::auth::session::Session;
use crate::core::random::token_generator::generate_token;
use crate::core::auth::player::Player;

const TOKEN_SIZE: usize = 32usize;

pub struct SessionManager {
    session_lifespan: Duration,
    stored_sessions: HashMap<String, Session>,
}

impl SessionManager {
    pub fn new(session_lifespan: Duration) -> SessionManager {
        SessionManager {
            session_lifespan,
            stored_sessions: HashMap::new(),
        }
    }

    pub fn create_session_for_player(&mut self, player: Player) -> String {
        let session: Session = Session::new(player);

        let session_token: String = generate_token(TOKEN_SIZE);
        let session_token_clone: String = session_token.clone();

        self.stored_sessions.insert(session_token, session);

        session_token_clone
    }

    pub fn use_session(&mut self, session_token: &str) -> Option<String> {
        if !(self.stored_sessions.contains_key(session_token)) {
            println!("Invalid session token detected.");

            return None;
        }

        let session: &Session = self.stored_sessions.get(session_token).unwrap();

        if self.session_lifespan <= session.get_created_on().elapsed() {
            println!("Session has expired. Creating a new one.");
            let session: Option<Session> = self.stored_sessions.remove(session_token);

            match session {
                Some(instance) => {
                    let new_session_token: String = self.create_session_for_player(instance.expire());

                    Some(new_session_token)
                }
                None => {
                    None
                }
            }
        } else {
            println!("Session has been used, but it hasn't expired yet.");
            None
        }
    }
}