enum AuthenticationRepositoryError {
    NotFound,
    #[allow(dead_code)]
    TechnicalError,
    #[allow(dead_code)]
    OtherError,
}

/**
 * Authentication repository containing account credentials to handle access requests
 */
struct AuthenticationRepository();

impl AuthenticationRepository {
    async fn check_credentials(&self, email: &str, password: &str) -> Result<Vec<Account>, AuthenticationRepositoryError> {
        // TODO: add a new OAuth client

        AuthenticationRepositoryError::OtherError
    }
}
