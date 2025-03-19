use crate::AppState;
use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tower_sessions::Session;

pub fn main_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    auth_name: &'a str,
}

pub async fn home(session: Session) -> impl IntoResponse {
    let auth_name: String = session
        .get("auth_name")
        .await
        .unwrap()
        .unwrap_or("".to_string());

    let tmpl = HomeTemplate {
        title: "Posts Page",
        auth_name: &auth_name,
    };

    println!("{:?}", auth_name);

    Html(tmpl.render().unwrap())
}
