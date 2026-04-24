use crate::auth::customer::Customer;
use crate::database::ArcDynDatabase;
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerLoginRequest {
    pub access_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerLoginResponse {
    pub customer_id: String,
    pub access_code: String,
    pub display_name: Option<String>,
    pub jwt_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLoginResponse {
    pub account_id: String,
    pub email: String,
    pub display_name: Option<String>,
    pub jwt_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    exp: usize,
    iat: usize,
    iss: String,
    role: String,
    is_admin: bool,
}

pub struct AuthManager {
    pub db: ArcDynDatabase,
}

impl AuthManager {
    pub fn new(db: ArcDynDatabase) -> Self {
        Self { db }
    }

    pub async fn create_customer(
        &self,
        access_code: String,
        display_name: String,
    ) -> Result<String, anyhow::Error> {
        let customer_id = uuid::Uuid::new_v4().hyphenated().to_string();

        self.db
            .create_customer(customer_id.clone(), access_code.clone(), display_name)
            .await?;

        info!("Created new customer with access code: {}", access_code);
        Ok(customer_id)
    }

    pub async fn verify_access_code(
        &self,
        access_code: String,
    ) -> Result<CustomerLoginResponse, anyhow::Error> {
        let customer = self.db.get_customer_by_access_code(&access_code).await?;

        let jwt_token = Self::generate_jwt_token(&customer.customer_id)
            .map_err(|e| anyhow::anyhow!("Failed to generate JWT: {}", e))?;

        info!("Customer verified successfully: {}", customer.access_code);

        Ok(CustomerLoginResponse {
            customer_id: customer.customer_id,
            access_code: customer.access_code,
            display_name: customer.display_name,
            jwt_token: Some(jwt_token),
        })
    }

    pub async fn create_account(
        &self,
        email: String,
        password: String,
        display_name: String,
        access_code: Option<String>,
    ) -> Result<String, anyhow::Error> {
        let account_id = uuid::Uuid::new_v4().hyphenated().to_string();

        let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        self.db
            .create_account(
                account_id.clone(),
                email.clone(),
                password_hash,
                Some(display_name),
            )
            .await?;

        // If an access code is provided, link it to this account
        if let Some(code) = access_code {
            match self.db.get_customer_by_access_code(&code).await {
                Ok(customer) => {
                    self.db
                        .link_access_code_to_account(&account_id, &customer.customer_id)
                        .await?;
                    info!("Linked access code {} to account {}", code, account_id);
                }
                Err(e) => {
                    error!("Could not find customer for access code {}: {}", code, e);
                    // Non-fatal: account is created, link is skipped
                }
            }
        }

        info!("Created new account with email: {}", email);
        Ok(account_id)
    }

    pub async fn verify_account_credentials(
        &self,
        email: String,
        password: String,
    ) -> Result<AccountLoginResponse, anyhow::Error> {
        let account = self.db.get_account_by_email(&email).await?;

        let valid = bcrypt::verify(&password, &account.password_hash)
            .map_err(|e| anyhow::anyhow!("bcrypt error: {}", e))?;

        if !valid {
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        let is_admin = self.db.is_account_admin(&account.account_id).await.unwrap_or(false);
        let jwt_token = Self::generate_account_jwt(&account.account_id, is_admin)
            .map_err(|e| anyhow::anyhow!("Failed to generate JWT: {}", e))?;

        info!("Account verified successfully: {}", account.email);

        Ok(AccountLoginResponse {
            account_id: account.account_id,
            email: account.email,
            display_name: account.display_name,
            jwt_token: Some(jwt_token),
        })
    }

    pub async fn get_customer_by_id(&self, customer_id: String) -> Option<Customer> {
        self.db.get_customer(&customer_id).await.ok()
    }

    pub fn generate_jwt_token(customer_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        use jsonwebtoken::{encode, EncodingKey, Header};

        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-key-change-in-production".to_string());

        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            sub: customer_id.to_string(),
            exp: now + 86400,
            iat: now,
            iss: "photos.network".to_string(),
            role: "customer".to_string(),
            is_admin: false,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn generate_account_jwt(account_id: &str, is_admin: bool) -> Result<String, jsonwebtoken::errors::Error> {
        use jsonwebtoken::{encode, EncodingKey, Header};

        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-key-change-in-production".to_string());

        let now = Utc::now().timestamp() as usize;
        let claims = JwtClaims {
            sub: account_id.to_string(),
            exp: now + 86400,
            iat: now,
            iss: "photos.network".to_string(),
            role: "account".to_string(),
            is_admin,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }

    fn access_code_regex() -> Regex {
        Regex::new(r"^[A-Z0-9]{6}$").unwrap()
    }

    pub fn validate_access_code(input: &str) -> bool {
        let regex = Self::access_code_regex();
        regex.is_match(input)
    }

    pub async fn update_last_login(&self, customer_id: String) -> Result<(), anyhow::Error> {
        self.db.update_last_login_for_customer(&customer_id).await
    }

    pub async fn update_last_login_account(&self, account_id: String) -> Result<(), anyhow::Error> {
        self.db.update_last_login_for_account(&account_id).await
    }

    /// Validates a JWT token and returns `(sub, role)`.
    pub fn validate_jwt_token(token: &str) -> Result<(String, String), anyhow::Error> {
        use jsonwebtoken::{decode, DecodingKey, Validation};

        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-secret-key-change-in-production".to_string());

        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| anyhow::anyhow!("Invalid JWT: {}", e))?;

        Ok((token_data.claims.sub, token_data.claims.role))
    }
}
