use rocket_contrib::databases::postgres::rows::Rows;
use rocket_contrib::databases::postgres::Error;

use crate::core::auth::authenticator::Authenticator;
use crate::core::auth::error::AuthenticationError;
use crate::core::auth::user::User;
use crate::database::Database;

struct PostgreSQLAuthenticator {
    database: Database,
}

impl PostgreSQLAuthenticator {
    pub fn new(database: Database) -> PostgreSQLAuthenticator {
        PostgreSQLAuthenticator { database }
    }

    fn check_rows_for_user_validity(&self, user: &User, rows: Rows) -> Result<bool, AuthenticationError> {
        if rows.len() == 0 {
            Ok(false)
        } else if rows.len() == 1 {
            Ok(true)
        } else {
            let error_message: String = String::from(format!(
                "Multiple results found for user with name '{}' and code '{}'",
                user.get_name(),
                user.get_code()
            ));

            Err(AuthenticationError::MultipleAccountsWithSameDetails(error_message))
        }
    }
}

impl Authenticator for PostgreSQLAuthenticator {
    fn is_user_valid(&self, user: &User) -> Result<bool, AuthenticationError> {
        let sql_query: &str = "SELECT * FROM player WHERE name=$1 AND code=$2";

        let result = self.database.0.query(
            sql_query,
            &[&String::from(user.get_name()), user.get_code()],
        );

        match result {
            Ok(rows) => self.check_rows_for_user_validity(user, rows),
            Err(_) => Err(AuthenticationError::FailedToAuthenticate(String::from(
                "Failed to retrieve data from database.",
            ))),
        }
    }
}
