use crate::AppState;
use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use axum_messages::Messages;

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/posts/create", get(create_post))
        .route("/posts", post(post_post))
}

#[derive(Template)]
#[template(path = "login.html")]
struct CreatePostTemplate<'a> {
    title: &'a str,
    messages: Vec<String>,
}

pub async fn create_post(messages: Messages) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message))
        .collect::<Vec<_>>();

    let tmpl = CreatePostTemplate {
        title: "Login Page",
        messages,
    };

    Html(tmpl.render().unwrap())
}

pub async fn post_post() -> impl IntoResponse {
    ""
}
