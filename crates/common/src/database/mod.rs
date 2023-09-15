use async_trait::async_trait;
use std::error::Error;

use crate::auth::user::User;

#[async_trait]
pub trait Database {
    async fn setup(&mut self) -> Result<(), Box<dyn Error>>;
    async fn create_user(&self, user: &User) -> Result<(), Box<dyn Error>>;
    async fn update_email(&self, email: &str, user_id: &str) -> Result<(), Box<dyn Error>>;
    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>>;
}
