use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn auth_middleware(session: Session, request: Request, next: Next) -> impl IntoResponse {
    match session
        .get::<String>("auth_name")
        .await
        .expect("Session getting user failed")
    {
        Some(_) => next.run(request).await,
        None => Redirect::to("/login").into_response(),
    }
}
