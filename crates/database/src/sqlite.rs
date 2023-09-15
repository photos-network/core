use async_trait::async_trait;
use common::auth::user::User;
use common::database::Database;
use sqlx::types::time::OffsetDateTime;
use sqlx::Row;
use sqlx::SqlitePool;
use std::error::Error;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqliteDatabase {
    pub pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(db_url: &str) -> Self {
        let pool = SqlitePool::connect(db_url).await.unwrap();

        SqliteDatabase { pool }
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    async fn setup(&mut self) -> Result<(), Box<dyn Error>> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }

    async fn create_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)";
        let id = Uuid::new_v4().hyphenated().to_string();
        info!("create new user with id `{}`.", id);
        sqlx::query(query)
            .bind(id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(&user.lastname)
            .bind(&user.firstname)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_email(&self, email: &str, user_id: &str) -> Result<(), Box<dyn Error>> {
        let query = "UPDATE users SET email = $1 WHERE uuid = $2";

        sqlx::query(query)
            .bind(email)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let query = "SELECT uuid, email, password, lastname, firstname FROM users";

        let res = sqlx::query(query);

        let rows = res.fetch_all(&self.pool).await?;

        let users = rows
            .iter()
            .map(|row| User {
                uuid: row.get("uuid"),
                email: row.get("email"),
                password: row.get("password"),
                lastname: row.get("lastname"),
                firstname: row.get("firstname"),
                is_locked: false,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                last_login: None,
            })
            .collect();

        Ok(users)
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn create_user_should_succeed(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/create_user_should_succeed.sqlite",
        )
        .await;

        // when
        for i in 0..3 {
            let user = User {
                uuid: uuid::Uuid::new_v4().hyphenated().to_string(),
                email: format!("test_{}@photos.network", i),
                password: Some("unsecure".into()),
                lastname: Some("Stuermer".into()),
                firstname: Some("Benjamin".into()),
                is_locked: false,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                last_login: None,
            };

            // when
            let _ = db.create_user(&user).await;
        }

        // then
        let count = sqlx::query("SELECT COUNT(*) AS 'count!' FROM users")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.get::<i32, _>("count!"), 3);

        Ok(())
    }

    #[sqlx::test]
    async fn create_already_existing_user_should_fail(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/create_already_existing_user_should_fail.sqlite",
        )
        .await;

        // when
        let uuid = uuid::Uuid::new_v4().hyphenated().to_string();
        let user = User {
            uuid,
            email: "info@photos.network".into(),
            password: Some("unsecure".into()),
            lastname: Some("Stuermer".into()),
            firstname: Some("Benjamin".into()),
            is_locked: false,
            created_at: OffsetDateTime::now_utc(),
            updated_at: None,
            last_login: None,
        };

        // then
        let result1 = db.create_user(&user.clone()).await;
        assert!(result1.is_ok());

        let result2 = db.create_user(&user.clone()).await;
        assert!(result2.is_err());

        let count = sqlx::query("SELECT COUNT(*) AS 'count!' FROM users")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.get::<i32, _>("count!"), 1);

        Ok(())
    }

    #[sqlx::test]
    async fn update_email_should_succeed(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/update_email_should_succeed.sqlite",
        )
        .await;

        // when
        let result = db
            .update_email(
                "security@photos.network".into(),
                "570DC079-664A-4496-BAA3-668C445A447".into(),
            )
            .await;

        // then
        assert!(result.is_ok());
        let count = sqlx::query("SELECT email FROM users LIMIT 1")
            .fetch_one(&pool)
            .await?;
        assert_eq!(count.get::<String, _>("email"), "security@photos.network");

        Ok(())
    }

    #[sqlx::test]
    async fn update_email_to_existing_should_fail(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;

        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("0D341AD3-D38F-455F-8411-E25186665FC5")
            .bind("security@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;

        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/update_email_to_existing_should_fail.sqlite",
        )
        .await;

        // when
        let result = db
            .update_email(
                "security@photos.network".into(),
                "570DC079-664A-4496-BAA3-668C445A447".into(),
            )
            .await;

        // then
        assert!(result.is_err());

        let rows = sqlx::query("SELECT email FROM users")
            .fetch_all(&pool)
            .await?;
        assert_eq!(rows[0].get::<String, _>("email"), "info@photos.network");
        assert_eq!(rows[1].get::<String, _>("email"), "security@photos.network");

        Ok(())
    }

    #[sqlx::test]
    async fn get_users_should_succeed(pool: SqlitePool) -> sqlx::Result<()> {
        // given
        sqlx::query("INSERT INTO users (uuid, email, password, lastname, firstname) VALUES ($1, $2, $3, $4, $5)")
            .bind("570DC079-664A-4496-BAA3-668C445A447")
            .bind("info@photos.network")
            .bind("unsecure")
            .bind("Stuermer")
            .bind("Benjamin")
            .execute(&pool).await?;
        let db = SqliteDatabase::new(
            "target/sqlx/test-dbs/database/sqlite/tests/get_users_should_succeed.sqlite",
        )
        .await;

        // when
        let users = db.get_users().await.unwrap();

        // then
        assert_eq!(users.clone().len(), 1);
        assert_eq!(
            users.get(0).unwrap().uuid,
            "570DC079-664A-4496-BAA3-668C445A447"
        );

        Ok(())
    }
}
