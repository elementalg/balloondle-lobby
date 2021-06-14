use crate::core::auth::error::AuthenticationError;
use crate::core::auth::user::User;

pub trait Authenticator {
    fn are_details_valid(&self, name: &String, code: &u32) -> Result<bool, AuthenticationError>;
    fn get_user_for_details(&self, name: &String, code: &u32) -> Result<User, AuthenticationError>;
    fn create_user_with_details(&self, name: &String, code: &u32) -> User;
}