use rocket::http::Status;
use rocket::response::content;
use serde_json::Error;

use crate::adapter::postgresql_authenticator::PostgreSQLAuthenticator;
use crate::core::auth::authenticator::Authenticator;
use crate::core::auth::error::AuthenticationError;
use crate::core::auth::user::User;
use crate::database::Database;
use crate::error::ApiError;

#[get("/status")]
pub fn status() -> Status {
    Status::Accepted
}

#[get("/user/startup?<name>&<code>")]
pub fn user_startup(database: Database, name: String, code: u32) -> content::Json<String> {
    let authenticator = get_authenticator_implementation(database);

    let user_validity = authenticator.as_ref().are_details_valid(&name, &code);

    match user_validity {
        Ok(is_valid) => {
            if is_valid {
                let user = authenticator.get_user_for_details(&name, &code);

                get_content_for_user_result(user)
            } else {
                generate_content_for_error(ApiError::InvalidUserDetails)
            }
        }
        Err(_) => {
            generate_content_for_error(ApiError::FailedToProcessRequest)
        }
    }
}

fn get_authenticator_implementation(database: Database) -> Box<dyn Authenticator> {
    Box::new(PostgreSQLAuthenticator::new(database))
}

fn get_content_for_user_result(user_result: Result<User, AuthenticationError>) -> content::Json<String> {
    match user_result {
        Ok(user) => {
            let user_json = serde_json::to_string(&user);

            match user_json {
                Ok(valid_user_json) => {
                    content::Json(valid_user_json)
                }
                Err(_) => {
                    generate_content_for_error(ApiError::FailedToProcessRequest)
                }
            }
        }
        Err(_) => {
            generate_content_for_error(ApiError::InvalidUserDetails)
        }
    }
}

fn generate_content_for_error(error: ApiError) -> content::Json<String> {
    let error_data: String = String::from(format!("{{ 'error': '{}' }}", error as u32));

    content::Json(error_data)
}