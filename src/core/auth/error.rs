pub enum AuthenticationError {
    FailedToAuthenticate(String),
    MultipleAccountsWithSameDetails(String),
}
