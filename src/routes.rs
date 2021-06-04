use rocket::http::Status;

#[get("/status")]
pub fn status() -> Status {
    Status::Accepted
}