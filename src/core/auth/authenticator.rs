use crate::core::auth::user::User;
use crate::core::auth::error::AuthenticationError;

pub trait Authenticator {
    fn is_user_valid(&self, user: &User) -> Result<bool, AuthenticationError>;
}