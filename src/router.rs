use crate::{
    AppState,
    auth_middleware::auth_middleware,
    handlers::{posts_handler::posts_router, users_handler::users_router},
};
use axum::{Router, middleware};

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(posts_router())
        .layer(middleware::from_fn(auth_middleware))
        .merge(users_router())
}
