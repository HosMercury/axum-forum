use crate::{AppState, utils::validation_errors};
use askama::Template;
use axum::{
    Form, Router,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_flash::{Flash, IncomingFlashes};
use serde::Deserialize;
use validator::Validate;

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login))
        .route("/login", post(post_login))
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    title: &'a str,
    flashes: Vec<String>,
}

pub async fn login(flashes: IncomingFlashes) -> impl IntoResponse {
    let flashes = flashes
        .iter()
        .map(|(_, msg)| msg.to_string())
        .collect::<Vec<String>>();

    let tmpl = LoginTemplate {
        title: "Login Page",
        flashes,
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

pub async fn post_login(flash: Flash, Form(data): Form<LoginData>) -> (Flash, Redirect) {
    // validate the upcoming data
    if let Err(errors) = data.validate() {
        let error_messages = validation_errors(errors);

        let mut flash = flash;

        for error in error_messages {
            flash = flash.error(error);
        }

        (flash, Redirect::to("/login"))
    } else {
        (flash, Redirect::to("/"))
    }
}
