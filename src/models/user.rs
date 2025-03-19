use chrono::{DateTime, Local};
use password_auth::{generate_hash, verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as, query_scalar};
use tokio::task;

use crate::handlers::users_handler::{LoginData, RegisterData};

#[derive(Serialize, Deserialize, Clone, Default, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub created_at: DateTime<Local>,
}

impl User {
    pub async fn login(pool: &PgPool, data: LoginData) -> anyhow::Result<User> {
        println!("{:?}", data.email);

        let user: User = query_as("SELECT * FROM users WHERE email = $1 ")
            .bind(&data.email)
            .fetch_one(pool)
            .await
            .unwrap();

        let user_password = user.password.clone();
        task::spawn_blocking(move || verify_password(&data.password, &user_password)).await??;

        Ok(user)
    }

    pub async fn register(pool: &PgPool, data: RegisterData) -> anyhow::Result<Self> {
        let hashed_password: String =
            task::spawn_blocking(move || generate_hash(&data.password)).await?;

        let user: User =
            query_as("INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *")
                .bind(&data.name)
                .bind(&data.email)
                .bind(hashed_password)
                .fetch_one(pool)
                .await?;

        Ok(user)
    }

    pub async fn email_exists(pool: &PgPool, email: &str) -> anyhow::Result<bool> {
        let count: i64 = query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(count > 0)
    }
}
