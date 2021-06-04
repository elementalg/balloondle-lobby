use rocket_contrib::databases::postgres;

#[database("balloondle_db")]
pub struct BalloondleDatabase(pub(crate) postgres::Connection);