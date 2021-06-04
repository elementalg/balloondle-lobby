use std::sync::{Arc, Mutex};

use rand::Rng;
use rocket::response::content;
use rocket::State;

use crate::core::auth::user::User;
use crate::database::Database;