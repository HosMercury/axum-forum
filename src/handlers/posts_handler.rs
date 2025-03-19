use crate::{AppState, models::post::Post, utils::validation_errors};
use askama::Template;
use axum::{
    Form, Router, debug_handler,
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use validator::Validate;

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/", get(posts))
        .route("/posts/create", get(create_post))
        .route("/posts", post(post_post))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    auth_name: &'a str,
    messages: Vec<String>,
    posts: Vec<Post>,
}

pub async fn posts(
    session: Session,
    messages: Messages,
    State(AppState { pool, .. }): State<AppState>,
) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message))
        .collect::<Vec<_>>();

    let auth_name: String = session
        .get("auth_name")
        .await
        .unwrap()
        .unwrap_or("".to_string());

    let posts = Post::all(&pool).await.unwrap_or(vec![]);

    let tmpl = HomeTemplate {
        title: "Posts Page",
        auth_name: &auth_name,
        messages,
        posts,
    };

    println!("{:?}", auth_name);

    Html(tmpl.render().unwrap())
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
        title: "Create Post",
        messages,
        auth_name: &auth_name,
    };

    Html(tmpl.render().unwrap())
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PostData {
    #[validate(length(min = 8, message = "Title must be at least 8 characters long"))]
    pub title: String,

    #[validate(length(min = 50, message = "Content must be at least 50 characters long"))]
    pub content: String,
}

#[debug_handler]
pub async fn post_post(
    messages: Messages,
    State(AppState { pool, .. }): State<AppState>,
    Form(data): Form<PostData>,
) -> impl IntoResponse {
    if let Err(errors) = data.validate() {
        let error_messages = validation_errors(errors);

        let mut messages = messages;

        for error in error_messages {
            messages = messages.error(error);
        }

        Redirect::to("/posts/create")
    } else {
        if let Err(_) = Post::create(&pool, &data).await {
            messages.error("something went wrong");
            Redirect::to("/posts/create")
        } else {
            messages.success("Post created successfully");
            Redirect::to("/")
        }
    }
}
