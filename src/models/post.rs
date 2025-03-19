use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query, query_as};

use crate::handlers::posts_handler::PostData;

#[derive(Serialize, Deserialize, Clone, Default, FromRow, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<chrono::Local>,
}

impl Post {
    pub async fn create(pool: &PgPool, data: &PostData) -> anyhow::Result<()> {
        query("INSERT INTO posts (title, content) VALUES ($1, $2)")
            .bind(&data.title)
            .bind(&data.content)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn all(pool: &PgPool) -> anyhow::Result<Vec<Post>> {
        let posts = query_as("SELECT * FROM posts").fetch_all(pool).await?;

        Ok(posts)
    }

    pub async fn find(pool: &PgPool, id: i32) -> anyhow::Result<Post> {
        let post = query_as("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(post)
    }
}
