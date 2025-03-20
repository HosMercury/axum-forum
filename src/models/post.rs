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
    pub user_id: i32,
    pub user_name: String,
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
        let posts = query_as(
            "SELECT users.name AS user_name, 
            post.* FROM posts join users ON user_id = posts.user_id",
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn find(pool: &PgPool, id: i32) -> anyhow::Result<Post> {
        let post = query_as("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(post)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn update(pool: &PgPool, id: i32, data: &PostData) -> anyhow::Result<()> {
        query("UPDATE posts SET title = $1, content = $2 WHERE id = $3")
            .bind(&data.title)
            .bind(&data.content)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
