use crate::AppState;
use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

pub fn main_router() -> Router<AppState> {
    Router::new().route("/", get(home))
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    title: &'a str,
}

pub async fn home() -> impl IntoResponse {
    let tmpl = HomeTemplate { title: "Home Page" };

    Html(tmpl.render().unwrap())
}
