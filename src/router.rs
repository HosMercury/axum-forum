use crate::{
    AppState,
    handlers::{
        main_handler::main_router, posts_handler::posts_router, users_handler::users_router,
    },
};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(main_router())
        .merge(users_router())
        .merge(posts_router())
}
