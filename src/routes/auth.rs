use rocket::http::Status;
use rocket::response::status::{Accepted, BadRequest, Created};

use crate::adapter::postgresql_authenticator::PostgreSQLAuthenticator;
use crate::core::auth::authenticator::Authenticator;
use crate::core::auth::error::AuthenticationError;
use crate::core::auth::user::User;
use crate::database::Database;
use crate::error::ApiError;

#[post("/user/signup?<name>")]
pub fn user_signup(database: Database, name: String) -> Result<Created<String>, Status> {
    let authenticator = get_authenticator_implementation(database);
    let code = rand::random::<u32>();

    authenticator.create_user_with_details(&name, &code);

    let user = User::new(name.as_str(), code);

    let user_json = serde_json::to_string(&user);

    match user_json {
        Ok(valid_user_json) => Ok(Created(String::from(""), Some(valid_user_json))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/login?<name>&<code>")]
pub fn user_login(
    database: Database,
    name: String,
    code: u32,
) -> Result<Result<Accepted<String>, BadRequest<String>>, Status> {
    let authenticator = get_authenticator_implementation(database);

    let user_validity = authenticator.as_ref().are_details_valid(&name, &code);

    match user_validity {
        Ok(is_valid) => {
            if is_valid {
                let user = authenticator.get_user_for_details(&name, &code);

                Ok(get_content_for_user_result(user))
            } else {
                Err(Status::BadRequest)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

fn get_authenticator_implementation(database: Database) -> Box<dyn Authenticator> {
    Box::new(PostgreSQLAuthenticator::new(database))
}

fn get_content_for_user_result(
    user_result: Result<User, AuthenticationError>,
) -> Result<Accepted<String>, BadRequest<String>> {
    match user_result {
        Ok(user) => {
            let user_json = serde_json::to_string(&user);

            match user_json {
                Ok(valid_user_json) => Ok(Accepted(Some(valid_user_json))),
                Err(_) => Err(BadRequest(Some(get_json_for_error(
                    ApiError::FailedToProcessRequest,
                )))),
            }
        }
        Err(_) => Err(BadRequest(Some(get_json_for_error(
            ApiError::InvalidUserDetails,
        )))),
    }
}

fn get_json_for_error(error: ApiError) -> String {
    let error_data: String = String::from(format!("{{ 'error' : '{}' }}", error as u32));

    error_data
}
