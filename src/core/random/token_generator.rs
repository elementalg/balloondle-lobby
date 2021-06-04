use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_token(size: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}