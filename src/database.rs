use rocket_contrib::databases::postgres;

#[database("balloondle_db")]
pub struct Database(pub(crate) postgres::Connection);