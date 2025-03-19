use chrono::{DateTime, Local};
use password_auth::{generate_hash, verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query, query_as, query_scalar};
use tokio::task;
use tower_sessions::Session;

use crate::handlers::users_handler::{LoginData, RegisterData};

#[derive(Serialize, Deserialize, Clone, Default, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub created_at: DateTime<Local>,
}

impl User {
    pub async fn login(pool: &PgPool, data: LoginData, session: &Session) -> anyhow::Result<bool> {
        println!("{:?}", data.email);

        let user: User = query_as("SELECT * FROM users WHERE email = $1 ")
            .bind(&data.email)
            .fetch_one(pool)
            .await
            .unwrap();

        task::spawn_blocking(move || verify_password(&data.password, &user.password)).await??;

        Ok(true)
    }

    pub async fn register(
        pool: &PgPool,
        data: RegisterData,
        session: &Session,
    ) -> anyhow::Result<bool> {
        let hashed_password: String =
            task::spawn_blocking(move || generate_hash(&data.password)).await?;

        query("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)")
            .bind(&data.name)
            .bind(&data.email)
            .bind(hashed_password)
            .execute(pool)
            .await?;

        Ok(true)
    }

    pub async fn email_exists(pool: &PgPool, email: &str) -> anyhow::Result<bool> {
        let exists: bool = query_scalar("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(true)
    }
}
