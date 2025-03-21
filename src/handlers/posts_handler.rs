use crate::models::user::User;
use crate::{AppState, models::post::Post, utils::validation_errors};
use askama::Template;
use axum::{
    Form, Router, debug_handler,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use sqlx::query;
use tower_sessions::Session;
use validator::Validate;

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/", get(posts))
        .route("/posts/create", get(create_post))
        .route("/posts", post(save_post))
        .route("/posts/{id}/update", post(update_post))
        .route("/posts/{id}/delete", get(delete_post))
        .route("/posts/{id}/edit", get(edit_post))
        .route("/posts/{id}", get(show_post))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    auth_user: User,
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

    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let posts = Post::all(&pool).await.unwrap_or_else(|_| vec![]);

    let tmpl = HomeTemplate {
        title: "Posts Page",
        auth_user,
        messages,
        posts,
    };

    Html(tmpl.render().unwrap()).into_response()
}

#[derive(Template)]
#[template(path = "create-post.html")]
struct CreatePostTemplate<'a> {
    title: &'a str,
    messages: Vec<String>,
    auth_user: User,
}

pub async fn create_post(messages: Messages, session: Session) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message))
        .collect::<Vec<_>>();

    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let tmpl = CreatePostTemplate {
        title: "Create Post",
        messages,
        auth_user,
    };

    Html(tmpl.render().unwrap()).into_response()
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PostData {
    #[validate(length(min = 8, message = "Title must be at least 8 characters long"))]
    pub title: String,

    #[validate(length(min = 50, message = "Content must be at least 50 characters long"))]
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct PostUserData {
    #[validate(length(min = 8, message = "Title must be at least 8 characters long"))]
    pub title: String,

    #[validate(length(min = 50, message = "Content must be at least 50 characters long"))]
    pub content: String,

    pub user_id: i32,
}

#[debug_handler]
pub async fn save_post(
    messages: Messages,
    session: Session,
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
        let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

        if let Err(e) = Post::create(&pool, &data, auth_user.id).await {
            messages.error("something went wrong");
            Redirect::to("/posts/create")
        } else {
            messages.success("Post created successfully");
            Redirect::to("/")
        }
    }
}

#[derive(Template)]
#[template(path = "show-post.html")]
struct ShowPostTemplate<'a> {
    title: &'a str,
    auth_user: User,
    post: Post,
}

pub async fn show_post(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let post_result = Post::find(&pool, id).await;

    if let Err(_) = post_result {
        return Redirect::to("/").into_response();
    }

    let tmpl = ShowPostTemplate {
        title: "Posts Page",
        auth_user,
        post: post_result.unwrap(),
    };

    Html(tmpl.render().unwrap()).into_response()
}

pub async fn delete_post(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let post = match Post::find(&pool, id).await {
        Ok(post) => post,
        Err(_) => return Redirect::to("/").into_response(),
    };

    if post.user_id != auth_user.id {
        return Redirect::to("/").into_response();
    }

    if let Err(_) = Post::delete(&pool, id).await {
        return Redirect::to("/").into_response();
    }

    Redirect::to("/").into_response()
}

#[derive(Template)]
#[template(path = "edit-post.html")]
struct EditPostTemplate<'a> {
    title: &'a str,
    messages: Vec<String>,
    auth_user: User,
    post: Post,
}

pub async fn edit_post(
    messages: Messages,
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message))
        .collect::<Vec<_>>();

    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let post = match Post::find(&pool, id).await {
        Ok(post) => post,
        Err(_) => return Redirect::to("/").into_response(),
    };

    let tmpl = EditPostTemplate {
        title: "Create Post",
        messages,
        auth_user,
        post,
    };

    Html(tmpl.render().unwrap()).into_response()
}

pub async fn update_post(
    messages: Messages,
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Path(id): Path<i32>,
    Form(data): Form<PostData>,
) -> impl IntoResponse {
    if let Err(errors) = data.validate() {
        let error_messages = validation_errors(errors);

        let mut messages = messages;

        for error in error_messages {
            messages = messages.error(error);
        }

        return Redirect::to("/posts/create").into_response();
    }

    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let post = match Post::find(&pool, id).await {
        Ok(post) => post,
        Err(_) => return Redirect::to("/").into_response(),
    };

    if post.user_id != auth_user.id {
        return Redirect::to("/").into_response();
    }

    if let Err(_) = Post::update(&pool, id, &data).await {
        messages.error("something went wrong");
        return Redirect::to("/posts/create").into_response();
    } else {
        messages.success("Post created successfully");
        return Redirect::to("/").into_response();
    }
}
