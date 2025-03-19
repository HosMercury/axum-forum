use crate::AppState;
use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use axum_messages::Messages;
use tower_sessions::Session;

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/posts/create", get(create_post))
        .route("/posts", post(post_post))
}

#[derive(Template)]
#[template(path = "create-post.html")]
struct CreatePostTemplate<'a> {
    title: &'a str,
    messages: Vec<String>,
    auth_name: &'a str,
}

pub async fn create_post(messages: Messages, session: Session) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message))
        .collect::<Vec<_>>();

    let auth_name = session
        .get("auth_name")
        .await
        .unwrap()
        .unwrap_or("".to_string());

    let tmpl = CreatePostTemplate {
        title: "Login Page",
        messages,
        auth_name: &auth_name,
    };

    Html(tmpl.render().unwrap())
}

pub async fn post_post() -> impl IntoResponse {
    ""
}
