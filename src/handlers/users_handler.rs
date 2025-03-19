use crate::{AppState, models::user::User, utils::validation_errors};
use askama::Template;
use axum::{
    Form, Router, debug_handler,
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use serde::Deserialize;
use tower_sessions::Session;
use validator::Validate;

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/login", post(post_login))
        .route("/register", get(register))
        .route("/register", post(post_register))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
}

#[debug_handler]
pub async fn login() -> impl IntoResponse {
    let tmpl = LoginTemplate {
        title: "Login Page",
    };

    Html(tmpl.render().unwrap())
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
}

pub async fn register() -> impl IntoResponse {
    let tmpl = RegisterTemplate {
        title: "Register Page",
    };

    Html(tmpl.render().unwrap())
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

pub async fn post_login(
    session: Session,
    messages: Messages,
    State(AppState { pool, .. }): State<AppState>,
    Form(data): Form<LoginData>,
) -> Redirect {
    // validate the upcoming data
    if let Err(errors) = data.validate() {
        let error_messages = validation_errors(errors);

        let mut messages = messages;

        for error in error_messages {
            messages = messages.error(error);
        }

        Redirect::to("/login")
    } else {
        // if the data is valid we want to check login in db
        let login = User::login(&pool, data, &session).await;

        if let Err(_) = login {
            return Redirect::to("/login");
        }

        Redirect::to("/")
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterData {
    #[validate(length(min = 4, message = "Name must be at least 4 characters long"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    confirm_password: String,
}

pub async fn post_register(
    session: Session,
    State(AppState { pool, .. }): State<AppState>,
    Form(data): Form<RegisterData>,
) -> Redirect {
    // validate the upcoming data

    if let Err(errors) = data.validate() {
        let error_messages = validation_errors(errors);

        Redirect::to("/register")
    } else {
        // if the data is valid we want to register the user
        let exists = User::email_exists(&pool, &data.email).await;

        if let Err(_) = exists {
            return Redirect::to("/register");
        }

        let register = User::register(&pool, data, &session).await;

        if let Err(_) = register {
            return (Redirect::to("/register"));
        }

        (Redirect::to("/"))
    }
}
