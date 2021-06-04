use rocket_contrib::databases::postgres::rows::{Row, Rows};

use crate::core::auth::authenticator::Authenticator;
use crate::core::auth::error::AuthenticationError;
use crate::core::auth::user::User;
use crate::database::Database;

const USER_COLUMNS_COUNT: usize = 2usize;

pub struct PostgreSQLAuthenticator {
    database: Database,
}

impl PostgreSQLAuthenticator {
    pub fn new(database: Database) -> PostgreSQLAuthenticator {
        PostgreSQLAuthenticator { database }
    }

    fn check_rows_for_user_validity(
        &self,
        name: &str,
        code: &u32,
        rows: Rows,
    ) -> Result<bool, AuthenticationError> {
        if rows.len() == 0 {
            Ok(false)
        } else if rows.len() == 1 {
            Ok(true)
        } else {
            let error_message: String = String::from(format!(
                "Multiple results found for user with name '{}' and code '{}'",
                name, code
            ));

            Err(AuthenticationError::MultipleAccountsWithSameDetails(
                error_message,
            ))
        }
    }

    fn get_user_from_rows(
        &self,
        name: &str,
        code: &u32,
        rows: Rows,
    ) -> Result<User, AuthenticationError> {
        if rows.len() == 0 {
            let error_message: String = String::from(format!(
                "Tried to get data for a non existing user with name '{}' and code '{}'",
                name, code
            ));

            Err(AuthenticationError::FailedToAuthenticate(error_message))
        } else if rows.len() == 1 {
            let row = rows.iter().next();

            match row {
                Some(row) => self.create_user_from_row(row),
                None => {
                    let error_message: String =
                        String::from(format!("Failed to retrieve details from database"));

                    Err(AuthenticationError::FailedToAuthenticate(error_message))
                }
            }
        } else {
            let error_message: String = String::from(format!(
                "Tried to get data for multiple accounts with the same details, name '{}', code '{}'",
                name, code
            ));

            Err(AuthenticationError::MultipleAccountsWithSameDetails(
                error_message,
            ))
        }
    }

    fn create_user_from_row(&self, row: Row) -> Result<User, AuthenticationError> {
        if row.len() == USER_COLUMNS_COUNT {
            let name: String = row.get(0);
            let code: u32 = row.get(1);

            Ok(User::new(name.as_str(), code))
        } else {
            Err(AuthenticationError::FailedToAuthenticate(format!(
                "Row contains '{}' columns, expected '{}'",
                row.len(),
                USER_COLUMNS_COUNT
            )))
        }
    }
}

impl Authenticator for PostgreSQLAuthenticator {
    fn are_details_valid(&self, name: &String, code: &u32) -> Result<bool, AuthenticationError> {
        let sql_query: &str = "SELECT * FROM player WHERE name=$1 AND code=$2";

        let result = self.database.0.query(sql_query, &[name, code]);

        match result {
            Ok(rows) => self.check_rows_for_user_validity(name.as_str(), &code, rows),
            Err(_) => Err(AuthenticationError::FailedToAuthenticate(String::from(
                "Failed to retrieve data from database.",
            ))),
        }
    }

    fn get_user_for_details(&self, name: &String, code: &u32) -> Result<User, AuthenticationError> {
        let sql_query: &str = "SELECT * FROM player WHERE name=$1 AND code=$2";

        let result = self.database.0.query(sql_query, &[name, code]);

        match result {
            Ok(rows) => self.get_user_from_rows(name.as_str(), code, rows),
            Err(_) => Err(AuthenticationError::FailedToAuthenticate(String::from(
                "Failed to retrieve data from database.",
            ))),
        }
    }
}
